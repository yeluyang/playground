package main

import (
	"fmt"
	"os"
	"os/signal"
	"strconv"
	"sync"
	"syscall"
	"time"

	"github.com/urfave/cli"

	"github.com/allegro/bigcache"
)

const (
	SCALE = 1024
	KB    = 1 * SCALE
	MB    = 1 * SCALE * KB
	GB    = 1 * SCALE * MB
)

var verbose bool
var caches []*bigcache.BigCache

var app = cli.App{
	Name: "bigcache-alloc",
	Flags: []cli.Flag{
		&cli.BoolFlag{
			Name:        "verbose",
			Aliases:     []string{"V"},
			Value:       false,
			Destination: &verbose,
		},
		&cli.BoolFlag{
			Name:    "pre-allocate",
			Aliases: []string{"a"},
			Usage:   "should allocate all capacity need?",
		},
		&cli.IntFlag{
			Name:    "cache-number",
			Aliases: []string{"c"},
			Usage:   "number of caches",
		},
		&cli.IntFlag{
			Name:    "item-total",
			Aliases: []string{"t"},
			Usage:   "number of total items in all cache",
		},
		&cli.IntFlag{
			Name:    "item-size",
			Aliases: []string{"s"},
			Usage:   "size of item, unit is 'KB'",
			Value:   8,
		},
		&cli.IntSliceFlag{
			Name:    "item-number",
			Aliases: []string{"n"},
			Usage:   "number of total items in per cache",
		},
	},
	Action: func(c *cli.Context) error {
		if !c.IsSet("item-number") && !c.IsSet("item-total") {
			return fmt.Errorf("item-number and item-total must be given at least one")
		}

		if c.IsSet("item-total") {
			if !c.IsSet("cache-number") {
				if err := c.Set("cache-number", strconv.Itoa(1)); err != nil {
					return err
				}
			}
			if c.IsSet("item-number") {
				total := 0
				for n := range c.IntSlice("item-number") {
					total += n
				}
				if c.Int("item-total") < total {
					return fmt.Errorf("value of item-total must be greater than or equal to sum of values of item-number")
				}
			}
		} else {
			total := 0
			for n := range c.IntSlice("item-number") {
				total += n
			}
			if err := c.Set("item-total", strconv.Itoa(total)); err != nil {
				return err
			}

			if !c.IsSet("cache-number") {
				if err := c.Set("cache-number", strconv.Itoa(len(c.IntSlice("item-number")))); err != nil {
					return err
				}
			} else {
				if c.Int("cache-number") != len(c.IntSlice("item-number")) {
					return fmt.Errorf("value of cache-number must be equal to length of value-list of item-number when item-total is not set")
				}
			}
		}

		return run(c.Int("cache-number"), c.Int("item-total"), c.Int("item-size")*KB, c.Bool("pre-allocate"), c.IntSlice("item-number"))
	},
}

func BigCacheConfigFrom(itemTotal int, itemMaxSize int, preAlloc bool, verbose bool) bigcache.Config {
	// XXX: never evict
	config := bigcache.DefaultConfig(200 * 365 * 24 * time.Hour)

	config.Verbose = verbose

	// `+1` ensure shardsNumUpLimit > 0
	shardsNumUpLimit := uint(itemTotal/100) + 1
	config.Shards = int(getNearestPowerOf2(shardsNumUpLimit))
	if config.Shards > 128 {
		config.Shards = 128
	}

	if preAlloc {
		config.MaxEntriesInWindow = itemTotal + 1
	} else {
		config.MaxEntriesInWindow = itemTotal / 10
	}
	config.MaxEntrySize = itemMaxSize

	// `+1` ensure config.HardMaxCacheSize > 0
	config.HardMaxCacheSize = ((itemTotal * itemMaxSize) / MB) + 1

	return config
}

func getNearestPowerOf2(a uint) uint {
	if a <= 0 {
		panic("input expected should >0")
	}
	if (a & (a - 1)) == 0 {
		return a
	}
	r := uint(1)
	for (r << 1) < a {
		r <<= 1
	}
	return r
}

func run(cacheTotal int, itemTotal int, itemSize int, preAlloc bool, itemNums []int) error {
	fmt.Printf("run with arguments: cacheTotal=%d, itemTotal=%d, itemSize=%d, preAlloc=%t, items=%+v\n", cacheTotal, itemTotal, itemSize, preAlloc, itemNums)
	if len(itemNums) != cacheTotal {
		remainTotalItems := itemTotal
		for _, num := range itemNums {
			remainTotalItems -= num
		}
		remainCaches := cacheTotal - len(itemNums)
		remainAvgItems := remainTotalItems / remainCaches
		for i := 0; i < remainCaches; i++ {
			itemNums = append(itemNums, remainAvgItems)
		}
	}

	type helper struct {
		index     int
		itemTotal int
		err       error
	}

	caches = make([]*bigcache.BigCache, len(itemNums))
	ch := make(chan *helper, len(itemNums))
	b := make([]byte, itemSize)
	var wg sync.WaitGroup
	wg.Add(len(itemNums))
	for i, itemNum := range itemNums {
		go func(i, itemNum int) {
			defer wg.Done()
			config := BigCacheConfigFrom(itemNum, itemSize, preAlloc, verbose)
			fmt.Printf("config=%+v\n", config)

			var err error
			caches[i], err = bigcache.NewBigCache(config)
			if err != nil {
				ch <- &helper{err: err}
				return
			}

			for k := 0; k < itemNum; k++ {
				err = caches[i].Set(strconv.Itoa(k), b)
				if err != nil {
					ch <- &helper{err: err}
					return
				}
			}
			ch <- &helper{
				index:     i,
				itemTotal: itemNum,
			}
		}(i, itemNum)
	}

	wg.Wait()
	close(ch)

	for h := range ch {
		if h.err != nil {
			return h.err
		} else {
			fmt.Printf("add %d item successfully, size expected: %fGB, actual={len=%fGB, capacity=%fGB}\n",
				h.itemTotal,
				float64(h.itemTotal*itemSize)/float64(GB),
				float64(caches[h.index].Len())/float64(GB),
				float64(caches[h.index].Capacity())/float64(GB),
			)
		}
	}

	return nil
}

func main() {

	errCH := make(chan error, 1)
	go func() {
		if err := app.Run(os.Args); err != nil {
			errCH <- err
		}
	}()

	sig := make(chan os.Signal, 2)
	signal.Notify(sig, os.Interrupt, syscall.SIGTERM)

	for {
		select {
		case err := <-errCH:
			fmt.Printf("%s", err)
			os.Exit(1)
		case <-sig:
			os.Exit(0)
		default:
			time.Sleep(10 * time.Millisecond)
		}
	}
}

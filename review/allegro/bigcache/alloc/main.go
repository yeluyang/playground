package main

import (
	"fmt"
	"strconv"
	"time"

	"github.com/allegro/bigcache"
)

const (
	SCALE = 1024
	KB    = 1 * SCALE
	MB    = 1 * SCALE * KB
	GB    = 1 * SCALE * MB
)

func BigCacheConfigFrom(itemTotal, itemMaxSize int) bigcache.Config {
	// XXX: never evict
	config := bigcache.DefaultConfig(200 * 365 * 24 * time.Hour)

	config.Verbose = false

	// `+1` ensure shardsNumUpLimit > 0
	shardsNumUpLimit := uint(itemTotal/100) + 1
	config.Shards = int(getNearestPowerOf2(shardsNumUpLimit))
	if config.Shards > 128 {
		config.Shards = 128
	}

	// init 10 entry in each shard, 10 is equal to `bigcache.minimumEntriesInShard`
	// `bigcache.minimumEntriesInShard` ensure number of entries in initialized shard > 0
	config.MaxEntriesInWindow = 10 * config.Shards
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

func main() {
	var itemTotal = 10 * 10000
	var itemSize = 8 * KB

	config := BigCacheConfigFrom(itemTotal, itemSize)
	fmt.Printf("config=%+v\n", config)

	cache, err := bigcache.NewBigCache(config)
	if err != nil {
		panic(err)
	}

	b := make([]byte, config.MaxEntrySize)
	for i := 0; err == nil; i++ {
		err = cache.Set(strconv.Itoa(i), b)
		if err != nil {
			fmt.Printf("err=%s\n", err)
		}
	}

	for {
		fmt.Printf("sleeping\n")
		time.Sleep(1 * time.Hour)
	}
}

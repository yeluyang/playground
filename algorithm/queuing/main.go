package main

import (
	"encoding/json"
	"fmt"
	"os"

	"github.com/yeluyang/playground/algorithm/queuing/pkg"

	"github.com/urfave/cli/v2"
)

var costs []int

type Processor struct {
	TPS   float64
	State *pkg.ProcessorState
}

func NewProcessor(tps float64, others int, qps float64) *Processor {
	return &Processor{
		TPS:   tps,
		State: pkg.NewProcessorState(tps, others, qps),
	}
}

func (p *Processor) String() string {
	s, err := json.MarshalIndent(p, "", "\t")
	if err != nil {
		panic(err)
	}
	return string(s)
}

func NewProcessorSet(tpss []float64, qps float64) []*Processor {
	processors := make([]*Processor, len(tpss))

	for i := range tpss {
		processors[i] = NewProcessor(tpss[i], len(tpss), qps)
	}

	return processors
}

func main() {
	app := &cli.App{
		Name: "queuing",
		Flags: []cli.Flag{
			&cli.IntSliceFlag{
				Name:     "costs",
				Aliases:  []string{"c"},
				Required: true,
			},
		},
		Before: func(c *cli.Context) error {
			costs = c.IntSlice("costs")
			return nil
		},
		Commands: []*cli.Command{
			&cli.Command{
				Name: "calc",
				Flags: []cli.Flag{
					&cli.Float64Flag{
						Name:    "qps",
						Aliases: []string{"q"},
						Value:   100.0,
					},
				},
				Action: func(c *cli.Context) error {
					tpss := make([]float64, len(costs))
					for i := range costs {
						tpss[i] = 1000 / float64(costs[i])
					}
					qps := c.Float64("qps")
					ps := NewProcessorSet(tpss, qps)
					if s, err := json.MarshalIndent(ps, "", "\t"); err != nil {
						return err
					} else {
						fmt.Printf("%s\n", s)
						return nil
					}
				},
			},
			&cli.Command{
				Name: "sim",
				Flags: []cli.Flag{
					&cli.Float64Flag{
						Name:    "duration",
						Aliases: []string{"d"},
						Value:   60,
					},
				},
				Action: func(c *cli.Context) error {
					// TODO
					return nil
				},
			},
		},
	}
	if err := app.Run(os.Args); err != nil {
		fmt.Printf("error occured: %s", err)
		os.Exit(1)
	}
}

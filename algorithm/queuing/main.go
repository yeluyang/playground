package main

import (
	"encoding/json"
	"fmt"
	"os"

	"github.com/yeluyang/playground/algorithm/queuing/pkg"

	"github.com/urfave/cli/v2"
)

type Processor struct {
	Speed float64
	State *pkg.ProcessorState
}

func NewProcessor(speed float64, others int, qps float64, seconds float64) *Processor {
	return &Processor{
		Speed: speed,
		State: pkg.NewProcessorState(speed, others, qps, seconds),
	}
}

type ProcessorSet []*Processor

func NewProcessorSet(speeds []float64, qps float64, seconds float64) ProcessorSet {
	processors := make(ProcessorSet, len(speeds))

	for i := range speeds {
		processors[i] = NewProcessor(speeds[i], len(speeds), qps, seconds)
	}

	return processors
}

func (ps *ProcessorSet) String() string {
	s, err := json.MarshalIndent(ps, "", "\t")
	if err != nil {
		panic(err)
	}
	return string(s)
}

func (p *Processor) String() string {
	s, err := json.MarshalIndent(p, "", "\t")
	if err != nil {
		panic(err)
	}
	return string(s)
}

func main() {
	app := &cli.App{
		Name: "queuing",
		Flags: []cli.Flag{
			&cli.Float64SliceFlag{
				Name:    "speeds",
				Aliases: []string{"s"},
				Value:   cli.NewFloat64Slice(50.0, 50.0),
			},
			&cli.Float64Flag{
				Name:    "qps",
				Aliases: []string{"q"},
				Value:   100.0,
			},
			&cli.Float64Flag{
				Name:    "duration",
				Aliases: []string{"d"},
				Value:   60,
			},
		},
		Action: func(c *cli.Context) error {
			speeds := c.Float64Slice("speeds")
			qps := c.Float64("qps")
			seconds := c.Float64("duration")
			ps := NewProcessorSet(speeds, qps, seconds)
			fmt.Printf("%s\n", ps)
			return nil
		},
	}
	if err := app.Run(os.Args); err != nil {
		fmt.Printf("error occured: %s", err)
	}
}

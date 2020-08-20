package main

import (
	"encoding/json"
	"fmt"
	"os"
	"time"

	"github.com/urfave/cli/v2"
	"github.com/yeluyang/playground/algorithm/queuing/pkg"
)

var costs []time.Duration

func main() {
	app := &cli.App{
		Name: "queuing",
		Flags: []cli.Flag{
			&cli.StringSliceFlag{
				Name:     "costs",
				Aliases:  []string{"c"},
				Required: true,
			},
		},
		Before: func(c *cli.Context) error {
			costStrs := c.StringSlice("costs")
			costs = make([]time.Duration, len(costStrs))
			for i := range costStrs {
				if d, err := time.ParseDuration(costStrs[i]); err != nil {
					return fmt.Errorf("failed to parse costs: %s", err)
				} else {
					costs[i] = d
				}
			}
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
					qps := c.Float64("qps")
					ps := pkg.NewProcessorSet(costs)
					for i := range ps {
						ps[i].Calc(qps)
					}
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
					&cli.Int64Flag{
						Name:    "users",
						Aliases: []string{"u"},
						Value:   10,
					},
					&cli.StringFlag{
						Name:    "duration",
						Aliases: []string{"d"},
						Value:   "1s",
					},
				},
				Action: func(c *cli.Context) error {
					users := c.Int64("users")
					duration, err := time.ParseDuration(c.String("duration"))
					if err != nil {
						return fmt.Errorf("failed to parse duration: %s", err)
					}
					sim := pkg.NewSimulator(costs, uint64(users))
					sim.Run(duration)
					if s, err := json.MarshalIndent(sim, "", "\t"); err != nil {
						return err
					} else {
						fmt.Printf("%s\n", s)
						return nil
					}
				},
			},
		},
	}
	if err := app.Run(os.Args); err != nil {
		fmt.Printf("error occured: %s", err)
		os.Exit(1)
	}
}

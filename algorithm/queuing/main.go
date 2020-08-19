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
					ps := pkg.NewProcessorSet(costs, qps)
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

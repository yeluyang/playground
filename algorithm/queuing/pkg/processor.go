package pkg

import (
	"encoding/json"
	"time"
)

type Processor struct {
	cost  time.Duration
	TPS   float64
	State *ProcessorState
}

func NewProcessor(cost time.Duration, others int, qps float64) *Processor {
	tps := float64(time.Second) / float64(cost)
	return &Processor{
		cost:  cost,
		TPS:   tps,
		State: NewProcessorState(tps, others, qps),
	}
}

func (p *Processor) String() string {
	s, err := json.MarshalIndent(p, "", "\t")
	if err != nil {
		panic(err)
	}
	return string(s)
}

func NewProcessorSet(costs []time.Duration, qps float64) []*Processor {
	processors := make([]*Processor, len(costs))

	for i := range costs {
		processors[i] = NewProcessor(costs[i], len(costs), qps)
	}

	return processors
}

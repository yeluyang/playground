package pkg

import (
	"time"

	uuid "github.com/satori/go.uuid"
)

type Processor struct {
	ID    string
	Cost  time.Duration
	TPS   float64
	peers int
	State *ProcessorState
}

func NewProcessor(cost time.Duration, peers int) *Processor {
	return &Processor{
		ID:    uuid.NewV4().String(),
		Cost:  cost,
		TPS:   float64(time.Second) / float64(cost),
		peers: peers,
		State: nil,
	}
}

func (p *Processor) Calc(qps float64) {
	p.State = NewProcessorState(p.TPS, qps, p.peers)
}

func NewProcessorSet(costs []time.Duration) []*Processor {
	processors := make([]*Processor, len(costs))

	for i := range costs {
		processors[i] = NewProcessor(costs[i], len(costs))
	}

	return processors
}

package pkg

import (
	"math"
	"time"

	uuid "github.com/satori/go.uuid"
)

type ProcessorState struct {
	IncomeCeil  int
	IncomeFloor int

	RemainCeil  int
	RemainFloor int
}

func NewProcessorState(tps float64, qps float64, peers int) *ProcessorState {
	incomeCeil := math.Ceil(qps / float64(peers))
	incomeFloor := math.Floor(qps / float64(peers))

	return &ProcessorState{
		IncomeCeil:  int(incomeCeil),
		IncomeFloor: int(incomeFloor),
		RemainCeil:  int(math.Max(incomeCeil-tps, 0)),
		RemainFloor: int(math.Max(incomeFloor-tps, 0)),
	}
}

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

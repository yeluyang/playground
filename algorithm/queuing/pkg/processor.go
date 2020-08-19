package pkg

import "encoding/json"

type Processor struct {
	TPS   float64
	State *ProcessorState
}

func NewProcessor(tps float64, others int, qps float64) *Processor {
	return &Processor{
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

func NewProcessorSet(tpss []float64, qps float64) []*Processor {
	processors := make([]*Processor, len(tpss))

	for i := range tpss {
		processors[i] = NewProcessor(tpss[i], len(tpss), qps)
	}

	return processors
}

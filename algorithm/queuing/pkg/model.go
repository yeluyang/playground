package pkg

import (
	"encoding/json"
	"math"
)

type ProcessorState struct {
	IncomeCeil  int
	IncomeFloor int

	RemainCeil  int
	RemainFloor int
}

func NewProcessorState(tps float64, others int, qps float64) *ProcessorState {
	incomeCeil := math.Ceil(qps / float64(others))
	incomeFloor := math.Floor(qps / float64(others))

	return &ProcessorState{
		IncomeCeil:  int(incomeCeil),
		IncomeFloor: int(incomeFloor),
		RemainCeil:  int(math.Max(incomeCeil-tps, 0)),
		RemainFloor: int(math.Max(incomeFloor-tps, 0)),
	}
}

func (p *ProcessorState) String() string {
	s, err := json.MarshalIndent(p, "", "\t")
	if err != nil {
		panic(err)
	}
	return string(s)
}

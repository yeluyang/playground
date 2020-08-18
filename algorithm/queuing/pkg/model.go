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

	RemainsCeil  float64
	RemainsFloor float64
}

func NewProcessorState(speed float64, others int, qps float64, seconds float64) *ProcessorState {
	incomeCeil := math.Ceil(qps / float64(others))
	incomeFloor := math.Floor(qps / float64(others))

	remainCeil := math.Max(incomeCeil-speed, 0)
	remainFloor := math.Max(incomeFloor-speed, 0)

	return &ProcessorState{
		IncomeCeil:   int(incomeCeil),
		IncomeFloor:  int(incomeFloor),
		RemainCeil:   int(remainCeil),
		RemainFloor:  int(remainFloor),
		RemainsCeil:  remainCeil * seconds,
		RemainsFloor: remainFloor * seconds,
	}
}

func (p *ProcessorState) String() string {
	s, err := json.MarshalIndent(p, "", "\t")
	if err != nil {
		panic(err)
	}
	return string(s)
}

package pkg

import (
	"math"
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

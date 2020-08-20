package pkg

import (
	"fmt"
	"math"
	"sort"
	"time"
)

type SimUnit struct {
	Cost  time.Duration
	Queue uint64
	Timer time.Duration
}

func NewSimUnit(cost time.Duration) *SimUnit {
	return &SimUnit{
		Cost:  cost,
		Queue: 0,
		Timer: 0,
	}
}

func (s *SimUnit) NextTransformation() *time.Duration {
	if s.Queue == 0 {
		return nil
	} else {
		t := time.Duration(math.Abs(float64(s.Timer - s.Cost)))
		return &t
	}
}

func (s *SimUnit) Transform(sep time.Duration) bool {
	if s.Queue > 0 {
		s.Timer += sep
		if s.Timer >= s.Cost {
			s.Queue -= 1
			s.Timer -= s.Cost
			return true
		} else {
			return false
		}
	} else {
		s.Timer = 0
		return false
	}
}

func (s *SimUnit) Alloc() {
	s.Queue += 1
}

type Simulator struct {
	Units              []*SimUnit
	Users              int64
	RoundRobinBalancer int
}

func NewSimulator(costs []time.Duration, users int64) *Simulator {
	s := &Simulator{
		Users:              users,
		RoundRobinBalancer: 0,
	}

	s.Units = make([]*SimUnit, len(costs))
	for i := range costs {
		s.Units[i] = NewSimUnit(costs[i])
	}
	sort.SliceStable(s.Units, func(i, j int) bool {
		return s.Units[i].Cost < s.Units[j].Cost
	})

	s.Alloc(s.Users)

	return s
}

func (s *Simulator) NextTransformation() time.Duration {
	sep := time.Duration(1<<63 - 1)
	var t *time.Duration
	for i := range s.Units {
		if t = s.Units[i].NextTransformation(); t != nil && *t < sep {
			sep = *t
		}
	}
	if t == nil {
		panic("no next transformation")
	}
	return sep
}

func (s *Simulator) Transform(sep time.Duration) int64 {
	newReqs := int64(0)
	for i := range s.Units {
		if s.Units[i].Transform(sep) {
			newReqs += 1
		}
	}
	return newReqs
}

func (s *Simulator) Alloc(requests int64) {
	for i := int64(0); i < requests; i++ {
		s.Units[s.RoundRobinBalancer].Alloc()
		s.RoundRobinBalancer = (s.RoundRobinBalancer + 1) % len(s.Units)
	}
}

func (s *Simulator) Run(duartion time.Duration) {
	remain := duartion
	for remain >= time.Second {
		remain -= time.Second
		second := time.Second

		qps := int64(0)
		// strat sample in one second
		for second > 0 {
			sep := time.Duration(math.Min(float64(s.NextTransformation()), float64(second)))
			// collect new requests
			newReqs := s.Transform(sep)
			qps += newReqs

			// allocate new requests
			s.Alloc(newReqs)
			second -= sep
		}
		fmt.Printf("%d\n", qps)
	}
}

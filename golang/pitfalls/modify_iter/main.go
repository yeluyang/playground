package main

import (
	"encoding/json"
	"fmt"
)

type Foo struct {
	Num int
}

func main() {
	foos := []*Foo{&Foo{Num: 0}, nil, &Foo{Num: 2}}
	s, err := json.Marshal(foos)
	if err != nil {
		panic(err)
	}
	fmt.Printf("foos=%s\n", s)

	for _, f := range foos {
		if f != nil {
			f.Num += 1
		} else {
			f = &Foo{Num: 1}
		}
	}
	s, err = json.Marshal(foos)
	if err != nil {
		panic(err)
	}
	fmt.Printf("foos=%s\n", s)

	for i, f := range foos {
		if f == nil {
			f = &Foo{Num: 1}
			foos[i] = f
		}
		f.Num += 1
	}
	s, err = json.Marshal(foos)
	if err != nil {
		panic(err)
	}
	fmt.Printf("foos=%s\n", s)

	m := map[string]int{
		"1": 1,
		"2": 2,
		"3": 3,
	}
	for _, v := range m {
		v = v * 2
	}
	fmt.Printf("m=%#v\n", m)
	for k := range m {
		m[k] = m[k] * 2
	}
	fmt.Printf("m=%#v\n", m)
}

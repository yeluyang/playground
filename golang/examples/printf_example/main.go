package main

import "fmt"

type Structure struct {
	id string
	s1 *Structure
	s2 *Structure
}

func main() {
	s := Structure{
		id: "0",
		s1: nil,
		s2: &Structure{id: "1"},
	}
	fmt.Printf("v(s)=%v\n", s)
	fmt.Printf("+v(s)=%+v\n", s)
	fmt.Printf("#v(s)=%#v\n", s)
}

package main

import "fmt"

func main() {
	empty := []int{}
	m := map[string][]int{}
	m["empty"] = append(m["empty"], empty...)

	var null []int
	m["nil"] = append(m["nil"], null...)

	// even slice is empty or nil, but key `empty` and `null` have been created by append
	fmt.Printf("m=%#v, empty.len=%d, nil.len=%d", m, len(m["empty"]), len(m["nil"]))
}

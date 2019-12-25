package main

import "fmt"

func main() {
	a := []int{1, 2, 3, 4}
	for _, e := range a {
		e = e * 2
	}
	fmt.Printf("a=%#v\n", a)
	for i := range a {
		a[i] = a[i] * 2
	}
	fmt.Printf("a=%#v\n", a)
	m := map[string]int{
		"1": 1,
		"2": 2,
		"3": 3,
	}
	for k, v := range m {
		m[k] = v * 2
	}
	fmt.Printf("m=%#v\n", m)
	for k := range m {
		m[k] = m[k] * 2
	}
	fmt.Printf("m=%#v\n", m)
}

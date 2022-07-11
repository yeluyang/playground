package main

func main() {
	l := -1

	// panic: runtime error: makeslice: len out of range
	_ = make([]int, l)

	// panic: runtime error: makeslice: len out of range
	_ = make([]int, 0, l)
}

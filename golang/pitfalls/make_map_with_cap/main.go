package main

import "fmt"

func main() {
	m := make(map[string][]string, 10)
	fmt.Printf("make a map with capcity=10: %#v, len=%d", m, len(m))
}

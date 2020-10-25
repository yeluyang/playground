package main

import (
	"fmt"
	"sync"
)

func main() {
	add_external()
	add_internal()
}

func add_external() {
	// NOTE bug
	var wg sync.WaitGroup
	for i := 0; i < 5; i++ {
		wg.Add(1)
		go func(wg sync.WaitGroup, id int) {
			defer wg.Done()
			fmt.Printf("id=%d\n", id)
		}(wg, i)
	}
	wg.Wait()
}

func add_internal() {
	// NOTE bug
	var wg sync.WaitGroup
	for i := 0; i < 5; i++ {
		go func(wg sync.WaitGroup, id int) {
			wg.Add(1)
			defer wg.Done()
			fmt.Printf("id=%d\n", id)
		}(wg, i)
	}
	wg.Wait()
}

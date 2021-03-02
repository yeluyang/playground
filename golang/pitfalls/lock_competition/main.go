package main

import (
	"fmt"
	"sync"
	"time"
)

func main() {
	var m sync.Mutex
	m.Lock()

	var wg sync.WaitGroup
	wg.Add(3)
	for i := 0; i < 3; i++ {
		go func(i int) {
			defer wg.Done()
			fmt.Printf("routine-%d start\n", i)
			m.Lock()
			defer m.Unlock()
			fmt.Printf("routine-%d exit\n", i)
		}(i)
		time.Sleep(1 * time.Second)
	}

	time.Sleep(1 * time.Second)
	fmt.Println("release")
	m.Unlock()

	wg.Wait()
}

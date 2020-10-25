package main

import (
	"fmt"
	"time"
)

type Test struct {
	id string
}

func main() {
	ch := make(chan bool, 0)

	go sendWait(ch, 0)
	recvWait(ch, 0)

	go sendWait(ch, 0)
	recvLoop(ch, 4*time.Second)

	go sendWait(ch, 4*time.Second)
	recvLoop(ch, 0)

	go sendLoop(ch, 0)
	recvWait(ch, 4*time.Second)

	go sendLoop(ch, 4*time.Second)
	recvWait(ch, 0)

	// NOTE: dead lock when use both `sendLoop` and `recvLoop`
	// go sendLoop(ch, 0)
	// recvLoop(ch, 4*time.Second)
}

func sendWait(ch chan bool, d time.Duration) {
	time.Sleep(d)
	ch <- true
	fmt.Println("sent!")
}
func sendLoop(ch chan bool, d time.Duration) {
	time.Sleep(d)
	for {
		select {
		case ch <- true:
			fmt.Println("sent!")
			return
		default:
			fmt.Println("wait to send")
			time.Sleep(1 * time.Second)
		}
	}
}

func recvWait(ch chan bool, d time.Duration) {
	time.Sleep(d)
	<-ch
	fmt.Println("got!")
}
func recvLoop(ch chan bool, d time.Duration) {
	time.Sleep(d)
	for {
		select {
		case <-ch:
			fmt.Println("got!")
			return
		default:
			fmt.Println("wait to receive")
			time.Sleep(1 * time.Second)
		}
	}
}

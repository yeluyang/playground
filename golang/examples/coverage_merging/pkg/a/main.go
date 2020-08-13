package a

import (
	"fmt"

	"github.com/yeluyang/playground/golang/examples/coverage_merging/pkg/b"
)

func aOnly() {
	fmt.Printf("aOnly is invoked")
}

func callB() {
	fmt.Printf("callB is invoked")
	b.BPub()
}

package main

import (
	"fmt"
	"reflect"
	"unsafe"
)

func main() {
	resized := []int{1, 2, 3, 4}
	resizedHeader := (*reflect.SliceHeader)((unsafe.Pointer(&resized)))
	fmt.Printf("resized: s={val=%v, len=%d, cap=%d}, h=%#v\n", resized, len(resized), cap(resized), resizedHeader)
	Append(resized)
	resizedHeader = (*reflect.SliceHeader)((unsafe.Pointer(&resized)))
	fmt.Printf("resized: s={val=%v, len=%d, cap=%d}, h=%#v\n", resized, len(resized), cap(resized), resizedHeader)
	Add(resized)
	resizedHeader = (*reflect.SliceHeader)((unsafe.Pointer(&resized)))
	fmt.Printf("resized: s={val=%v, len=%d, cap=%d}, h=%#v\n", resized, len(resized), cap(resized), resizedHeader)

	noResize := make([]int, 0, 10)
	noResizeHeader := (*reflect.SliceHeader)((unsafe.Pointer(&noResize)))
	fmt.Printf("no-resize: s={val=%v, len=%d, cap=%d}, h=%#v\n", noResize, len(noResize), cap(noResize), noResizeHeader)
	Append(noResize)
	noResizeHeader = (*reflect.SliceHeader)((unsafe.Pointer(&noResize)))
	fmt.Printf("no-resize: s={val=%v, len=%d, cap=%d}, h=%#v\n", noResize, len(noResize), cap(noResize), noResizeHeader)
	Add(noResize)
	noResizeHeader = (*reflect.SliceHeader)((unsafe.Pointer(&noResize)))
	fmt.Printf("no-resize: s={val=%v, len=%d, cap=%d}, h=%#v\n", noResize, len(noResize), cap(noResize), noResizeHeader)
}

func Append(s []int) {
	s = append(s, 5)
	h := (*reflect.SliceHeader)((unsafe.Pointer(&s)))
	fmt.Printf("append: s={val=%v, len=%d, cap=%d}, h=%#v\n", s, len(s), cap(s), h)
}

func Add(s []int) {
	for i := range s {
		s[i] += 5
	}
	h := (*reflect.SliceHeader)((unsafe.Pointer(&s)))
	fmt.Printf("add: s={val=%v, len=%d, cap=%d}, h=%#v\n", s, len(s), cap(s), h)
}

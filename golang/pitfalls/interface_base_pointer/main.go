package main

import (
	"fmt"
	"reflect"
)

type Foo interface {
	Foo()
}

type Bar struct {
	name string
}

func (b Bar) Foo() {
	fmt.Println(b.name)
}

func main() {
	f := &Bar{name: "[BAR] not nil"}
	foo(f)

	f.name = "[BAR] not pointer"
	foo(*f)

	f = nil
	foo(f)

	foo(nil)
}

func foo(f Foo) {
	if f != nil {
		if reflect.ValueOf(f).Kind() != reflect.Ptr {
			fmt.Println("f.value is not pointer")
			f.Foo()
		} else if !reflect.ValueOf(f).IsNil() {
			fmt.Println("f.value is not nil")
			f.Foo()
		} else {
			fmt.Println("f.value is nil")
		}
	} else {
		fmt.Println("f is nil")
	}
}

package main

import (
	"github.com/yeluyang/playground/golang/examples/internal_pkg/pkg"
	"github.com/yeluyang/playground/golang/examples/internal_pkg/pkg/foo"
	"github.com/yeluyang/playground/golang/examples/internal_pkg/pkg/zoo"

	"fmt"
)

func main() {
	fmt.Println(pkg.Pkg())
	fmt.Println("=======")
	fmt.Println(foo.Foo())
	fmt.Println("=======")
	fmt.Println(zoo.Zoo())
}

package pkg

import (
	"fmt"

	"github.com/yeluyang/playground/golang/examples/internal_pkg/pkg/foo"
)

func Pkg() string {
	return fmt.Sprintf("pkg\n%s", foo.Foo())
}

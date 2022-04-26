package foo

import (
	"fmt"

	"github.com/yeluyang/playground/golang/examples/internal_pkg/pkg/foo/internal"
)

func Foo() string {
	return fmt.Sprintf("foo\n%s", internal.Internal())
}

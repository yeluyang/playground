package buzz

import (
	"fmt"

	"github.com/yeluyang/playground/golang/examples/internal_pkg/pkg/foo/internal/bar"
)

func Buzz() string {
	return fmt.Sprintf("buzz\n%s", bar.Bar())
}

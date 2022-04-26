package zoo

import (
	"fmt"

	"github.com/yeluyang/playground/golang/examples/internal_pkg/pkg/foo/internal"
)

func Zoo() string {
	return fmt.Sprintf("zoo\n%s", internal.Internal())
}

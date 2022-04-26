package internal

import (
	"fmt"

	"github.com/yeluyang/playground/golang/examples/internal_pkg/pkg/foo/internal/buzz"
)

func Internal() string {
	return fmt.Sprintf("internal\n%s", buzz.Buzz())
}

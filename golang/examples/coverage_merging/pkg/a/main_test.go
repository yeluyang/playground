package a

import (
	"os"
	"testing"
)

func TestAOnly(t *testing.T) {
	aOnly()
}

func TestCallB(t *testing.T) {
	callB()
}

func TestEnvOnly(t *testing.T) {
	if len(os.Getenv("ENV_ON")) == 0 {
		return
	}
	envOnly()
}

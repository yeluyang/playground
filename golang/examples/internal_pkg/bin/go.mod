module github.com/yeluyang/playground/golang/examples/internal_pkg/bin

go 1.17

replace github.com/yeluyang/playground/golang/examples/internal_pkg/pkg => ../pkg

require github.com/yeluyang/playground/golang/examples/internal_pkg/pkg v0.0.0-00010101000000-000000000000

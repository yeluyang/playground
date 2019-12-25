package main

import "fmt"

func main() {
	m := map[string][]string{}
	m["key"] = append(m["key"], "value")
	fmt.Printf("call append on an nil slice in non-nil map is safety: %#v", m)
}

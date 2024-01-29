//go:build ignore

package main

import (
	"log"
	"net/http"
)

func main() {
	log.Fatal(http.ListenAndServe("localhost:9000", http.FileServer(http.Dir("."))))
}

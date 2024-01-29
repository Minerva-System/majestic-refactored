//cd ../../ && cargo build --bin majestic-wasm --target=wasm32-unknown-unknown && cp target/wasm32-unknown-unknown/debug/majestic-wasm.wasm misc/web/majestic.wasm
//go:generate cp $GOROOT/misc/wasm/wasm_exec.js .
//go:generate env GOOS=js GOARCH=wasm go build -o main.wasm

package main

import (
	"fmt"
	"html"
	"log"
	// "os"
	"strings"
	"syscall/js"
	// "time"
)

func fatal(err error) {
	log.Fatal(err)
}

var (
	doc js.Value
	input js.Value
	bottom js.Value
	tty js.Value

	majestic js.Value
)


const cursor = "‸"

func tprint() func(b []byte, echo bool) (int, int) {
	return func(b []byte, echo bool) (int, int) {
		text := html.EscapeString(string(b))
		text = strings.ReplaceAll(text, "\003", "^C")
		text = strings.ReplaceAll(text, "\004", "^D")
		if echo {
			text = "<b>" + text + "</b>"
		}
		output := doc.Call("getElementById", "tty")
		inner := strings.TrimSuffix(output.Get("innerHTML").String(), cursor)
		if strings.HasSuffix(inner, "</b>") && strings.HasPrefix(text, "<b>") {
			inner = strings.TrimSuffix(inner, "</b>")
			text = strings.TrimPrefix(text, "<b>")
		}
		output.Set("innerHTML", js.ValueOf(inner+text+cursor))
		bottom.Call("scrollIntoView", js.ValueOf(false))
		return len(b), 0
	}
}

func main() {
	doc = js.Global().Get("document")
	input = doc.Call("getElementById", "input")
	bottom = doc.Call("getElementById", "bottom")
	tty = doc.Call("getElementById", "tty")

	// majestic = js.Global().Get("majestic").Get("exports")
	// majestic.Call("majestic_init")

	printer := tprint()

	tty.Call("addEventListener", "click", js.FuncOf(func(this js.Value, args[]js.Value) any {
		input.Call("focus")
		return nil
	}))
	
	// Start majestic

	ready := make(chan bool, 1)

	wakeup := func() {
		select {
		case ready <- true:
		default:
		}
	}

	keydown := js.FuncOf(func(this js.Value, args []js.Value) any {
		e := args[0]
		e.Call("preventDefault")
		key := e.Get("key").String()
		ctrl := e.Get("ctrlKey").Bool()
		shift := e.Get("shiftKey").Bool()

		// js.Global().Get("console").Call("log", key)
		
		switch key {
		default:
			if len(key) > 1 {
				return nil
			}
		case "Enter":
			key = "\n" // call interpreter here...
		case "Backspace":
			key = "\b"
		case "Escape":
			key = "\033"
		case "Tab":
			key = "\t"
		}
		c := key[0]
		if (shift || ctrl) && 'a' <= c && c <= 'z' {
			c += ('A' - 'a') & 0o377
		}
		if ctrl && c >= '@' {
			c -= '@'
		}
		// sys.TTY[curtty].WriteByte(c)
		s := string(c)
		if s != "" {
			printer([]byte(s), false)
		}
		wakeup()
		return nil
	})
	
	input.Call("addEventListener", "keydown", keydown)
	input.Call("focus")
	fmt.Printf("Ready.\n")

	// var timer *time.Timer
	// var lastTimer time.Time
	for {
		// sys.Wait()
		// if !sys.Timer.IsZero() && (lastTimer.IsZero() || lastTimer.After(sys.Timer)) {
		// 	d := time.Until(sys.Timer)
		// 	if timer == nil {
		// 		timer = time.AfterFunc(d, wakeup)
		// 	} else {
		// 		timer.Reset(d)
		// 	}
		// }
		<-ready
	}

	select {}
}

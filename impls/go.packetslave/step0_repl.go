package main

import (
	"fmt"
	"io"
	"strings"

	readline "github.com/chzyer/readline"
)

func READ(in string) string {
	return in
}

func EVAL(in string) string {
	return in
}

func PRINT(in string) string {
	return in
}

func rep(in string) string {
	return PRINT(EVAL(READ(in)))
}

func main() {
	l, err := readline.NewEx(&readline.Config{
		Prompt: "user> ",
	})

	if err != nil {
		panic(err)
	}
	defer l.Close()

	for {
		text, err := l.Readline()
		if err == readline.ErrInterrupt || err == io.EOF {
			break
		}
		if err != nil {
			panic(err)
		}

		text = strings.TrimRight(text, "\n")
		if err != nil {
			return
		}
		fmt.Println(rep(text))
	}
}

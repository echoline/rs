package main

import (
	"github.com/aichaos/rivescript-go"
	"time"
	"fmt"
	"net"
	"strings"
	"bufio"
	"os"
	"io/ioutil"
	"io"
	"regexp"
)

func main() {
	bot := rivescript.New(nil)

	err := bot.LoadDirectory("/alice/replies")
	if err != nil {
		fmt.Printf("failed to load replies\n")
		return
	}

	bot.SortReplies()

	bot.SetSubroutine("time", func(rs *rivescript.RiveScript, args []string) string {
		return time.Now().Format(time.RFC1123)
	})

	bot.SetSubroutine("today", func(rs *rivescript.RiveScript, args []string) string {
		return time.Now().Weekday().String()
	})

	bot.SetSubroutine("learn", func(rs *rivescript.RiveScript, args []string) string {
		xrs := args[0]
		s := strings.Split(strings.Join(args[1:], " "), "::::")
		if len(s) >= 2 {
			file, err := os.Open(xrs)
			found := false
			contents := ""
			if err == nil {
				reader := bufio.NewReader(file)
				for {
					m := "+ " + s[1] + "\n"
					line, err := reader.ReadString('\n')
				        if err != nil && err != io.EOF {
						break
					}

					contents += line
					if line == m {
						found = true
						contents += "- " + s[0] + "\n"
					}

					if err != nil {
						break
					}
				}
				file.Close()
			}
			if found == false {
				contents += "\n+ " + s[1] + "\n- " + s[0] + "\n"
			}
			data := []byte(contents)
			err = ioutil.WriteFile(xrs, data, 0644)
			if err != nil {
				return "error writing to " + xrs
			}
			if len(s) == 3 {
				return ""
			}
			bot.LoadFile(xrs)
			bot.SortReplies()
			return "Okay, I'll try to remember to respond, \"" + s[0] + "\" when you say, \"" + s[1] + "\""
		}
		return ""
	})

	l, err := net.Listen("unix", "/tmp/alice")
	if err != nil {
		fmt.Printf("unix socket listen failed\n")
		return
	}

	for {
		fd, err := l.Accept()
		if err != nil {
			fmt.Printf("unix socket accept failed\n")
			return
		}

		buf := make([]byte, 8192)
		_, err = fd.Read(buf)
		if err != nil {
			fmt.Printf("unix socket read failed\n")
			return
		}

		str := strings.TrimSpace(string(buf))
		s := strings.Split(str, "\007")
		fmt.Printf("%s: %s\n", s[0], s[1])
		sentences := regexp.MustCompile(`[\.\?\!]`).Split(s[1], -1)
		r := ""
		for _, a := range sentences {
			b := strings.TrimSpace(a)
			if len(b) > 0 {
				reply, _ := bot.Reply(s[0], b)
				r += reply + " "
			}
		}

		fmt.Printf("me: %s\n", r)

		_, err = fd.Write([]byte(r))
		if err != nil {
			fmt.Printf("unix socket write failed\n")
			return
		}
	}
}


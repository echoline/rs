package main

import (
	"flag"
	"fmt"
	"os"
	"os/signal"
	"syscall"
	"net"
	"strings"

	"github.com/bwmarrin/discordgo"
)

// Variables used for command line parameters
var (
	Token string
)

func init() {

	flag.StringVar(&Token, "t", "", "Bot Token")
	flag.Parse()
}

func main() {

	// Create a new Discord session using the provided bot token.
	dg, err := discordgo.New("Bot " + Token)
	if err != nil {
		fmt.Println("error creating Discord session,", err)
		return
	}

	// Register the messageCreate func as a callback for MessageCreate events.
	dg.AddHandler(messageCreate)

	// In this example, we only care about receiving message events.
	dg.Identify.Intents = discordgo.IntentsGuildMessages

	// Open a websocket connection to Discord and begin listening.
	err = dg.Open()
	if err != nil {
		fmt.Println("error opening connection,", err)
		return
	}

	// Wait here until CTRL-C or other term signal is received.
	fmt.Println("Bot is now running.  Press CTRL-C to exit.")
	sc := make(chan os.Signal, 1)
	signal.Notify(sc, syscall.SIGINT, syscall.SIGTERM, os.Interrupt, os.Kill)
	<-sc

	// Cleanly close down the Discord session.
	dg.Close()
}

// This function will be called (due to AddHandler above) every time a new
// message is created on any channel that the authenticated bot has access to.
func messageCreate(s *discordgo.Session, m *discordgo.MessageCreate) {

	// Ignore all messages created by the bot itself
	// This isn't required in this specific example but it's a good practice.
	if m.Author.ID == s.State.User.ID {
		return
	}

	msg := strings.ToLower(m.Content)

	if strings.Contains(msg, "alice") {
		msg = strings.TrimPrefix(msg, "alice")
		msg = strings.TrimLeft(msg, ":, ")

		fd, err := net.Dial("unix", "/tmp/alice")
		if err != nil {
			fmt.Printf("unix socket connect failed\n")
			return
		}

		fmt.Printf("%s: %s\n", m.Author.ID, msg)

		buf := m.Author.ID + "\007" + msg
		_, err = fd.Write([]byte(buf))
		if err != nil {
			fmt.Printf("write failed\n")
			return
		}

		out := make([]byte, 8192)
		n, err := fd.Read(out)
		if err != nil {
			fmt.Printf("read failed\n")
			return
		}

		fmt.Printf("me: %s\n", string(out[:n]))

		s.ChannelMessageSend(m.ChannelID, string(out[:n]))

		fd.Close()
	}
}


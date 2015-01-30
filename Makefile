all: irc purple one

irc: irc.c

one: one.c

purple: purple.c
	gcc -g -o purple purple.c `pkg-config --cflags --libs purple`

clean:
	rm -f *.o *~ purple irc one

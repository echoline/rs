#define _GNU_SOURCE

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <sys/un.h>
#include <netdb.h>

#define MY_SOCKET "/tmp/alice"

char *alice(char *msg, char *source) {
	// this function relies on alicesocket.py
	// as a backend.  alice is a pyaiml program
	// echobot can also default to a backup response system
	int s, t, len;
	char *ptr;
	struct sockaddr_un remote;
	char str[1024];
	ptr = strchr(msg,'\n');
	if (ptr)
		ptr[0] = '\0';
	sprintf(str,"%s\007%s",source,msg);

	if ((s = socket(AF_UNIX, SOCK_STREAM, 0)) == -1) {
		return "error in socket()";
	}

	remote.sun_family = AF_UNIX;
	strcpy(remote.sun_path, MY_SOCKET);
	len = strlen(remote.sun_path) + sizeof(remote.sun_family);
	if (connect(s, (struct sockaddr *)&remote, len) == -1) {
		return "error in connect()";
	}	

	if (send(s, str, strlen(str), 0) == -1) {
		return "error in send()";
	}

	if ((t=recv(s, str, 1024, 0)) > 0) {
		str[t] = '\0';
	}

	close(s);

	if (t < 1)
		return "error in recv()";
	msg = str;

	return msg;
}

int main(int argc, char **argv)
{
	if (argc > 1)
		printf ("%s\n", alice (argv[1], getenv("USER")));
}


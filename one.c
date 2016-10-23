#define _GNU_SOURCE

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <sys/un.h>
#include <netdb.h>

char *MY_SOCKET = NULL;

char *alice(char *msg, char *source) {
	// this function relies on alicesocket.py
	// as a backend.  alice is a pyaiml program
	// echobot can also default to a backup response system
	int s, t, len;
	char *ptr;
	struct sockaddr_un remote;
	static char str[1024];
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

	return str;
}

int main(int argc, char **argv)
{
	char *ptr;
	char *user = getenv("USER");
	int arg;

	for (arg = 1; arg < argc; arg++) {
		if (!strcmp(argv[arg], "-h")) {
			printf ("%s [-h] [-s SOCKET] [-u YOURNAME] \"your message\"\n",
				argv[0]);
			return 0;
		}
		if (!strcmp(argv[arg], "-u") && (++arg < argc)) {
			user = strdup(argv[arg]);
		} else if (!strcmp(argv[arg], "-s") && (++arg < argc)) {
			MY_SOCKET = strdup(argv[arg]);
		} else {
			ptr = argv[arg];
		}
	}

	if (argc > 1) {
		if (MY_SOCKET == NULL) {
			MY_SOCKET = strdup("/tmp/alice");
		}

		alice (ptr, "");
		ptr = alice(ptr, user);
		printf ("%s\n", ptr);
	}

	return 0;
}


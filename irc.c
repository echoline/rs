/* this is a very simple irc bot
 * 
 * Released under the GNU GPL, which you can go find elsewhere.
 *
 * eli.neoturbine.net
 */

#define _GNU_SOURCE

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <sys/un.h>
#include <arpa/inet.h>
#include <netdb.h>

// colors for inclusion in printf formats
#define RED     "\E[31m\E[1m"
#define GREEN   "\E[32m\E[1m"
#define YELLOW  "\E[33m\E[1m"
#define BLUE    "\E[34m\E[1m"
#define NORMAL  "\E[m"

typedef struct _args {
	char* url;	// name of irc server
	char* nick;	// name of bot
	char* channel;	// default channel
	int port;	// server port
	int fork;	// whether to enable forking 
	int verbose;	// whether to output all traffic
} args;

int sockfd;             // socket descriptor
int last = 4;		// last result so randomness won't repeat
char target[32];	//also a small buffer so we won't lose this
char source[50];	//where a message came from
char *tmp;		// temporary string pointer
int i;			// temporary int

void write_to_socket(int target, char *buff_to_send);
void parse_arguments ( int argc, char* argv[], args* argp );
int greet(char* target, char* source, int last, int sockfd);
char *alice(char* msg);
void defalice();
void usage( char* name ) {
	printf(BLUE "usage" NORMAL ":\n");
	printf("	%s " RED "-c \"#channel\"\n" NORMAL, name);
	printf("	%s " YELLOW "-n nick\n" NORMAL, name);
	printf("	%s " GREEN "-h host\n" NORMAL, name);
	printf("	%s " BLUE "-p port\n" NORMAL, name);
	printf("	%s " RED "-f" NORMAL " (enable forking)\n", name);
	printf("	%s " YELLOW "-v" NORMAL " (verbose)\n", name);
}

int main(argc, argv)
	int argc;
	char *argv[];
{
	FILE *sockFD;                   // socket file descriptor
	struct sockaddr_in dest_addr;   // client address
	struct hostent *host_addr;	// client address
	char buf[4096];                 // received data 
	char buf2[1024];                // extra buffer 
	char *tmp2;			// temporary string pointer
	size_t len;			// length of getline return
	int maxforks = 2;		// how many times can we fork
	int forks = 0;			// how many times have we forked
	char *pbuf;			// ptr to getline buf
	args myargs;			// struct for args
	myargs.channel = "#neoturbine";
	myargs.nick = "alice";
	myargs.url = "localhost";
	myargs.port = 6667;
	myargs.fork = 0;
	myargs.verbose = 0;
	int nicklen = 20;		// length of nicks
	char nick[nicklen];		// string of nick
	char newnick[nicklen];		// string of nick

	// error checking and arg parsing
	parse_arguments ( argc, argv, &myargs );

	sprintf(nick,"%s",myargs.nick);
	sprintf(newnick,"%s",nick);

	printf("u|2L I5 " BLUE "%s" NORMAL "\n", myargs.url);
	printf("P0|2t nUMB3r 1S " BLUE "%d\n" NORMAL, myargs.port);
	
	if ( ( host_addr = gethostbyname ( myargs.url ) ) == NULL )
	{
		fprintf ( stderr, RED "(4Nn07 |2e50Lv3 \"%s\"\n" NORMAL, myargs.url );
		exit ( 1 );
	}

	// fetch new socket each time connection quits.
	while(1) {
		// set the socket 
		if ((sockfd = socket(AF_INET, SOCK_STREAM, 0)) == -1)
		{
			fprintf ( stderr, RED "(4Nn07 53T S0(|<e7 \"%s\"\n" NORMAL, myargs.url );
			exit(1);
		}
		dest_addr.sin_family = AF_INET;          // host byte order
		dest_addr.sin_port = htons(myargs.port); // short, network byte order
		dest_addr.sin_addr = * ( ( struct in_addr * ) host_addr->h_addr );
		memset(&(dest_addr.sin_zero), '\0', 8);  // zero the rest of the struct

		// connect to the remote port
		if ((connect(sockfd, (struct sockaddr *)&dest_addr,
		    sizeof(struct sockaddr))) == -1)
		{
			fprintf ( stderr, RED "(4Nn07 (0|\\|N3(T t0 \"%s\"\n" NORMAL, myargs.url );
			exit(1);
		}

		// set file descriptor for reading
		sockFD = fdopen(sockfd,"r");
	
		// read one cycle from the server.  should be two for some servers.
		memset(buf,0,sizeof(buf));

		// write registration and join default channel
		sprintf(buf,"USER %s echobot %s :%s\n", nick, myargs.url, nick);
		write_to_socket(sockfd, buf);
		sprintf(buf,"NICK %s\n", nick);
		write_to_socket(sockfd, buf);
		sprintf(buf,"JOIN %s\n", myargs.channel);
		write_to_socket(sockfd, buf);
		memset(buf,0,sizeof(buf));
		printf(BLUE "H3lLo w0rlD\n" NORMAL);
		
		// loop and read a line until connection quits
		while(pbuf = buf, len = sizeof(buf), (getline(&pbuf, &len, sockFD) != -1)) {
			// PONG to PINGs from server
			if ( !strncmp(buf, "PING", 4) ) {
				buf[1] = 'O';
				if( !write(sockfd,buf,strlen(buf)) ) {
					perror("Write");
					exit(0);
				}
				continue;
			}
			// print input if -v
			if (myargs.verbose) {
				printf(YELLOW "%s" NORMAL, buf);
			}
			tmp = index(buf,' ')+1;
			// respond to PRIVMSG commands
			if ( !strncmp(tmp, "PRIVMSG", 7) ) {
				// decide where message came from
				tmp2 = buf+1;
				strncpy(source,tmp2,50);
				tmp2 = source;
				tmp2 = index(tmp2,'!');
				tmp2[0] = '\0';
				tmp = index(tmp,' ')+1;
				strncpy(target,tmp,32);
				tmp2 = index(target,' ');
				tmp2[0] = '\0';
				// if message did not come from a channel
				// set to source 
				if (target[0] != '#') {
					strncpy(target,source,32);
				}
				tmp = index(tmp,':')+1;
				// ctcp stuff
				if (tmp[0] == '\1') {
					tmp++;
					if (!myargs.verbose) {
                                                printf(YELLOW "%s" NORMAL, buf);
                                        }
					if ( !strncasecmp(tmp, "VERSION\1", 8)) {
						sprintf(buf,"NOTICE %s :\1VERSION i am echoline's bot\1\n",source);
						write_to_socket(sockfd,buf);
					}
					if ( !strncasecmp(tmp, "PING", 4)) {
						sprintf(buf,"NOTICE %s :\1%s", source, tmp);
						write_to_socket(sockfd,buf);
					}
					printf(BLUE "Oh no! a CTCP\n" NORMAL);
				// respond to directly addressed messages
				} else if ((!strncasecmp(tmp,nick,strlen(nick))) || (!strcmp(target,source))) {
					if (!myargs.verbose) {
						printf(YELLOW "%s" NORMAL, buf);
					}
					// remove bot nick
					if (strcmp(target,source) != 0)
						tmp += strlen(nick) + 1;
					// respond to any query
					strcpy(buf2, alice(tmp));
					if (strncasecmp(buf2,"error",5)) {
						if (strcmp(target,source) != 0)
							sprintf(buf,"PRIVMSG %s :%s: %s\n",target, source, buf2);
						else
							sprintf(buf,"PRIVMSG %s :%s\n",target, buf2);
						write_to_socket(sockfd, buf);
						printf(BLUE "G0 4sK aLiC3\n" NORMAL);
					}
				// respond to bot's name elsewhere in messages
				// commented out because it's annoying as fuck
/*				} else if (strcasestr(tmp,nick)) {
					if (!myargs.verbose) {
						printf(YELLOW "%s" NORMAL, buf);
					}
					// remove bot nick
					strcpy(buf2,tmp);
					//tmp2 = strstr(tmp,nick) + strlen(nick);
					//tmp = strstr(buf2,nick);
					//strcpy(tmp,tmp2);
					// respond
					strcpy(buf2, alice(buf2));
					if (strcmp(buf2,"default")) {
						sprintf(buf,"PRIVMSG %s :%s: %s\n",target, source, buf2);
						write_to_socket(sockfd, buf);
						printf(BLUE "G0 4sK aLiC3\n" NORMAL);
					}*/
				}
			// rejoin if kicked
			} else if ( !strncmp(tmp, "KICK", 4) ) {
				tmp = rindex(buf,' ');
				tmp[0] = '\0';
				tmp = rindex(buf,' ')+1;
				if ( !strncmp(tmp, nick, strlen(nick)) ) {
					printf(RED "Th4t g37s Me 5o |-|eaT3D!\n" NORMAL);
					sprintf(buf,"JOIN %s\n",target);
					write_to_socket(sockfd,buf);
				}
			// reregister if we haven't already
			} else if ( !strncmp(tmp, "451", 3) ) {
				sprintf(buf,"USER %s echobot %s :%s\n", nick, myargs.url, nick);
				write_to_socket(sockfd, buf);
				sprintf(buf,"NICK %s\n", nick);
				write_to_socket(sockfd, buf);
				sleep(1);
				sprintf(buf,"JOIN %s\n", myargs.channel);
				write_to_socket(sockfd, buf);
				printf(BLUE "H3lLo w0rlD\n" NORMAL);
			// change nick if current choice is taken
			} else if ( !strncmp(tmp, "433", 3) ) {
				printf(RED "D4mMi7, 1 li|<3D th47 oN3\n" NORMAL);
				srand( (unsigned int)time( NULL ) );
				sprintf(nick,"%s%d",newnick,rand() % 1000000);
				sprintf(buf,"NICK %s\n", nick);
				write_to_socket(sockfd, buf);
			// welcoming
			} else if ( !strncmp(tmp, "JOIN", 4) ) {
				tmp2 = buf+1;
				strncpy(source,tmp2,50);
				tmp2 = source;
				tmp2 = index(tmp2,'!');
				tmp2[0] = '\0';
				tmp = rindex(tmp,':')+1;
				tmp[strlen(tmp)-2] = '\0';
				strncpy(target,tmp,32);
				if (strcmp(source,nick) != 0) {
				//	sprintf(buf,"PRIVMSG %s :Welcome to %s, %s.  %s\n", tmp, target, source, greet);
				//	write_to_socket(sockfd, buf);
					printf(BLUE "H3y a n3wB!\n" NORMAL);
				}
			// for invites 
			} else if ( !strncmp(tmp, "INVITE", 6) ) {
				tmp = rindex(tmp,':')+1;
				sprintf(buf,"JOIN %s", tmp);
				write_to_socket(sockfd, buf);
				printf(BLUE "P4|27Y t1M3!!\n" NORMAL);
			}
			memset(buf,0,sizeof(buf));
		}
		fclose(sockFD);
		sleep(5);
	}
}

void write_to_socket(int target,char *buff_to_send)
{
	printf(GREEN "%s" NORMAL, buff_to_send);
	if( !write(target,buff_to_send,strlen(buff_to_send)) )
	{
		perror("Write");
		exit(0);
	}
}

void parse_arguments ( int argc, char* argv[], args* argp )
{
	int i = 0;

	while ( ( i = getopt ( argc, argv, "h:p:n:c:fv" ) ) != -1 )
	{
		switch ( i )
		{
			case 'h':
				argp->url = optarg;
				break;
			case 'p':
				argp->port = atoi ( optarg );
				break;
			case 'c':
				argp->channel = optarg;
				break;
			case 'n':
				argp->nick = optarg;
				break;
			case 'f':
				argp->fork = 1;
				break;
			case 'v':
				argp->verbose = 1;
				break;
			default:
				usage ( argv[0] );
	    			exit(1);
	    }
    }

    if ( argp->url == NULL || argp->port < 1 || argp->port > 65535 || argp->nick == \
NULL ||  argp->channel == NULL ) {
	    usage ( argv[0] );
	    exit(1);
    }
}

int greet(char *target, char *source, int last, int sockfd) {
	char buf[100];
	int i;
	srand( (unsigned int)time( NULL ) );
	//we don't want to say the same thing twice in a row
	while(last == (i = (rand() % 4)));
	last = i;
	switch(i) {
		case 0:
			sprintf(buf,"PRIVMSG %s :hello, %s\n",target,source);
			break;
		case 1:
			sprintf(buf,"PRIVMSG %s :%s: hey there\n",target,source);
			break;
		case 2:
			sprintf(buf,"PRIVMSG %s :what's up, %s?\n",target,source);
			break;
		case 3:
			sprintf(buf,"PRIVMSG %s :hi, %s\n",target,source);
			break;
		default:
			sprintf(buf,"PRIVMSG %s :whoops, how'd i get here??  you fucked up, echoline.\n",target);
	}
	write_to_socket(sockfd, buf);
	printf(BLUE "H3Ll0 tH3re\n" NORMAL);
	return (last);
}

char *alice(char *msg) {
	// this function relies on alicesocket.py
	// as a backend.  alice is a pyaiml program
	// echobot can also default to a backup response system
    int s, t, len;
    char *ptr;
    struct sockaddr_un remote;
    char str[1024];
    ptr = strchr(msg,'\n');
    ptr[0] = '\0';
    sprintf(str,"%s\007%s",source,msg);

    if ((s = socket(AF_UNIX, SOCK_STREAM, 0)) == -1) {
	defalice();
	return "error in socket()";
    }

    remote.sun_family = AF_UNIX;
    strcpy(remote.sun_path, "/tmp/rs");
    len = strlen(remote.sun_path) + sizeof(remote.sun_family);
    if (connect(s, (struct sockaddr *)&remote, len) == -1) {
	defalice();
	return "error in connect()";
    }	

    if (send(s, str, strlen(str), 0) == -1) {
	defalice();
	return "error in send()";
    }

    if ((t=recv(s, str, 1024, 0)) > 0) {
        str[t] = '\0';
    }

    close(s);

    if (t < 1)
	    return "error in recv()";
    msg = str;

    	// pause for typing reality-ness
	len = strlen(msg);
	if (len > 20)
		len = 20;
	for (t = 0; t < len; t++)
		usleep(99000);
    return msg;
}

void defalice() {
	char buf[100];
	// respond to informal pings
	if (strcasestr(tmp,"ping")) {
		sprintf(buf,"PRIVMSG %s :%s: pong\n",target,source);
		write_to_socket(sockfd, buf);
		printf(BLUE "PI|\\|g-P0nG\n" NORMAL);
	// respond to greetings
	}else if (strcasestr(tmp,"hi") || strcasestr(tmp,"hey") || strcasestr(tmp,"hello")) {
        	last = greet(target, source, last, sockfd);
	// respond to any query
	} else {
        	// random responses
	        srand( (unsigned int)time( NULL ) );
        	// we don't want to say the same thing twice in a row
	        while(last == (i = (rand() % 6)));
        	last = i;
        	switch(i) {
                	case 0:
                	        sprintf(buf,"PRIVMSG %s :i dunno?\n",target);
                	        break;
                	case 1:
                	        sprintf(buf,"PRIVMSG %s :%s: yes?\n",target, source);
                	        break;
                	case 2:
                	        sprintf(buf,"PRIVMSG %s :that's my nick, don't wear it out\n",target);
                	        break;
                	case 3:
                	        sprintf(buf,"PRIVMSG %s :%s: you called?\n",target, source);
                	        break;
                	case 4:
	                        sprintf(buf,"PRIVMSG %s :probably.\n", target);
        	                break;
        	        case 5:
        	                sprintf(buf,"PRIVMSG %s :i agree.\n", target);
        	                break;
        	        default:
       	 	                sprintf(buf,"PRIVMSG %s :whoops, how'd i get here??  you fucked up, echoline.\n",target);                                                }
		write_to_socket(sockfd, buf);
		printf(BLUE "50m3oNe 54iD m4h n1c|<\n" NORMAL);
	}
}

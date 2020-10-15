#include <netinet/in.h>
#include <stdio.h>
#include <sys/errno.h>
#include <sys/socket.h>
#include <sys/types.h>

int main(int argc, char** argv) {
    int sock = socket(AF_INET6, SOCK_DGRAM, 0);
    if (sock == -1) {
        printf("error %i\n", errno);
        return -1;
    }

    int val = 1;
    int error = setsockopt(sock, IPPROTO_IP, IP_RECVTOS, &val, sizeof(val));
    if (error == -1) {
        printf("error %i\n", errno);
        return -1;
    }

    printf("ok\n");
}

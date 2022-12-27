#include "stdio.h"
#include "unistd.h"
#include <stdlib.h>
#include "sys/wait.h"
int main(int argc, char* argv[]) {
    int status;
    char* arguments[argc];
    for (int i = 1; i < argc; i++) {
        arguments[i-1] = argv[i];
    }
    arguments[argc - 1] = NULL;
    int p = fork();
    if (p == 0) {
        execvp(argv[1], arguments);
        exit(1);
    }
    else {
        waitpid(p, &status, 0);
        if (status == 0) {
            printf("Success!\n");
        }
        else {
            printf("Failed, exit code = %d\n", status);
        }
    }
    return 0;
}
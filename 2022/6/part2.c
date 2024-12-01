#include <ctype.h>
#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>

#define DISTINCT 14

bool is_unique(char buffer[DISTINCT+1]) {
    for (int i = 0; i < DISTINCT; i++) {
        if (!isalpha(buffer[i])) {
            return false;
        }
        for (int j = 0; j < DISTINCT; j++) {
            if (buffer[i] == buffer[j] && i != j) {
                return false;
            }
        }
    }
    return true;
}

int main() {
    FILE *fptr = fopen("input.txt", "r");
    char buff[DISTINCT+1] = {};
    int i = 1;
    while (1) {
        for (int j = 0; j < DISTINCT-1; j++) {
            buff[j] = buff[j+1];
        }
        buff[DISTINCT-1] = fgetc(fptr);
        //scanf(" %c", &(buff[3]));
        //printf("Buffer: [%s]\n", buff);
        if (is_unique(buff)) {
            break;
        }
        i++;
    }
    printf("%d\n", i);

    return 0;
}

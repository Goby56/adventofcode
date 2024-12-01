#include <ctype.h>
#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>

bool is_unique(char buffer[5]) {
    for (int i = 0; i < 4; i++) {
        if (!isalpha(buffer[i])) {
            return false;
        }
        for (int j = 0; j < 4; j++) {
            if (buffer[i] == buffer[j] && i != j) {
                return false;
            }
        }
    }
    return true;
}

int main() {
    FILE *fptr = fopen("input.txt", "r");
    char buff[4+1] = {};
    int i = 1;
    while (1) {
        for (int j = 0; j < 3; j++) {
            buff[j] = buff[j+1];
        }
        buff[3] = fgetc(fptr);
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

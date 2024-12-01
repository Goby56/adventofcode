#include <stdio.h>
#include <stdlib.h>
#include <ctype.h>

int alpha_to_int(char c) {
    if (c >= 'a' && c <= 'z') {
        return c - 'a';
    } else if (c >= 'A' && c <= 'Z') {
        return c - 'A' + 26;
    }
    return 0;
}

int count_chars(char line[53]) {
    for (int i = 0; i < 53; i++) {
        if (line[i] == '\n') {
            return i;
        }
    }
    return 52;
}

char find_duplicate(char line[53], int comp_size) {
    for (int i = 0; i < comp_size; i++) {
        for (int j = comp_size; j < comp_size * 2; j++) {
            if (line[i] == line[j]) {
                return line[i];
            }
        }
    }
    return '?';
}

int main() {
    FILE *fptr;

    fptr = fopen("input.txt", "r");

    int tot_priority = 0;

    char line[53]; // 26*2+1=53 (lower- and uppercase characters & newline)
    while(fgets(line, sizeof(line), fptr) != NULL) {
        int comp_size = count_chars(line) / 2;
        char duplicate = find_duplicate(line, comp_size);
        tot_priority += alpha_to_int(duplicate) + 1;
    }

    printf("%d\n", tot_priority);

    return 0;
}

#include <ctype.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

void move(char **crates, int src, int dst, int amt) {
    int in_src = 0;
    int in_dst = 0;
    while (1) {
        if (crates[src][in_src] != ' ') {
            in_src++;
        }
        if (crates[dst][in_dst] != ' ') {
            in_dst++;
        }
        if (crates[src][in_src] == ' ' &&
            crates[dst][in_dst] == ' ') {
            break;
        }
    }
    strncpy(crates[dst] + in_dst, crates[src] + in_src - amt, amt);
    for (int i = 0; i < amt; i++) {
        crates[src][in_src - amt + i] = ' ';
    }
}

int main() {
    char *filename = "input.txt";
    FILE *fptr;
    fptr = fopen(filename, "r");
    char line[100] = {' '};
    int crate_count = 0;
    int stack_height = -1;
    while (fgets(line, sizeof(line), fptr)) {
        if (line[0] == '\n') {
            break;
        }
        for (int i = 0; i < sizeof(line); i++) {
            if (isalpha(line[i])) {
                crate_count++;
            }
        }
        stack_height++;
    }
    fseek(fptr, 0, SEEK_SET);
    int stack_count;
    if (strcmp(filename, "test.txt") == 0) {
        stack_count = 3;
    } else if (strcmp(filename, "input.txt") == 0) {
        stack_count = 9;
    }
    char **crates = malloc(stack_count * sizeof(char*));
    for (int s = 0; s < stack_count; s++) {
        crates[s] = malloc((crate_count+1) * sizeof(char));
    }
    for (int i = stack_height - 1; i >= 0; i--) {
        fgets(line, sizeof(line), fptr);
        for (int j = 0; j < stack_count; j++) {
            crates[j][i] = line[j*4 + 1];
        }
    }
    for (int s = 0; s < stack_count; s++) {
        for (int h = 0; h < crate_count; h++) {
            if (!isalpha(crates[s][h])) {
                crates[s][h] = ' ';
            }
        }
    }
    fgets(line, sizeof(line), fptr);
    fgets(line, sizeof(line), fptr);
    int src, dst, amt;
    while (fscanf(fptr, "move %d from %d to %d\n", &amt, &src, &dst) == 3) {
        move(crates, src-1, dst-1, amt);
    }

    for (int s = 0; s < stack_count; s++) {
        for (int h = 0; h < crate_count; h++) {
            if (!isalpha(crates[s][h+1]) && isalpha(crates[s][h])) {
                printf("%c", crates[s][h]);
            }
        }
    }
    printf("\n");
    fclose(fptr);

    return 0;
}

#include <stdio.h>
#include <stdlib.h>
#include <ctype.h>
#include <string.h>

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

int find_badge(char rucksacks[3][53]) {
    int item_counts[3][52] = {};
    for (int r = 0; r < 3; r++) {
        int rucksack_size = count_chars(rucksacks[r]);
        for (int i = 0; i < rucksack_size; i++) {
            item_counts[r][alpha_to_int(rucksacks[r][i])] += 1;
        }
    }
    for (int i = 0; i < 52; i++) {
        if (item_counts[0][i] != 0 &&
            item_counts[1][i] != 0 &&
            item_counts[2][i] != 0) {
            return i + 1;
        }
    }
    return 0;
}

int main() {
    FILE *fptr;
    fptr = fopen("input.txt", "r");
    char line[53]; // 26*2+1=53 (lower- and uppercase characters & newline)

    int tot_priority = 0;
    char group_rucksacks[3][53] = {};
    int i = 0;
    while(fgets(line, sizeof(line), fptr) != NULL) {
        strcpy(group_rucksacks[i % 3], line);
        if ((i % 3) - 3 == -1) {
            tot_priority += find_badge(group_rucksacks);
        }
        i++;
    }

    printf("%d\n", tot_priority);

    fclose(fptr);

    return 0;
}

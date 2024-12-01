#include <stdlib.h>
#include <stdio.h>
#include <string.h>

int *extract_ranges(char *str) {
    int *sec_ranges = malloc(4*sizeof(int));
    int start = 0, sec = 0;
    for (int i = 0; i < strlen(str); i++) {
        if (str[i] == '-' || str[i] == ',' || i == strlen(str) - 1) {
            char *sec_id = malloc(i - start + 1);
            strncpy(sec_id, str + start, i - start);
            sec_ranges[sec] = atoi(sec_id);
            sec++;
            start = i + 1;
        }
    }
    return sec_ranges;
}

int main() {
    FILE *fptr;
    fptr = fopen("input.txt", "r");
    char line[100]; // arbitrary long ranges
    
    int full_containments = 0;
    while (fgets(line, sizeof(line), fptr) != NULL) {
        int *sr = extract_ranges(line); // section ranges
        if (!(sr[0] < sr[2] ^ sr[1] >= sr[3]) ||
            !(sr[0] <= sr[2] ^ sr[1] > sr[3]) ||
            !(sr[0] <= sr[2] ^ sr[1] >= sr[3])) {
            full_containments++;
        }
    }
    fclose(fptr);

    printf("%d\n", full_containments);

    return 0;
}

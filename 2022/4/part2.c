#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>

bool intersects(int al, int au, int bl, int bu) {
    for (int i = al; i <= au; i++) {
        if (i >= bl && i <= bu) {
            return true;
        }
    }
    return false;
}

int main() {
    FILE *fptr;
    fptr = fopen("input.txt", "r");
    int sr[4];
    int intersections = 0;
    while (fscanf(fptr, "%d-%d,%d-%d", &sr[0], &sr[1], &sr[2], &sr[3]) == 4) {
        if (intersects(sr[0], sr[1], sr[2], sr[3])) {
            intersections++;
        }
    }
    fclose(fptr);
    printf("%d\n", intersections);
    return 0;
}

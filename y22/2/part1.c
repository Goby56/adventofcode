#include <stdio.h>
#include <stdlib.h>

char rps[3][2] = {"AX", "BY", "CZ"};

// +0 (lost), +3 (draw), +6 (win)
int scores[3][3] = {
    { 1+3, 2+6, 3+0},
    { 1+0, 2+3, 3+6},
    { 1+6, 2+0, 3+3}
};

int move_as_int(char move) {
    for (int h = 0; h < 3; h++) { // hand (e.g AX or BY)
        for (int s = 0; s < 2; s++) { // side (e.g A or X)
            if (move == rps[h][s]) {
                return h;
            }
        }
    }
    return -1;
}

int main() {
    FILE *fptr;

    fptr = fopen("input.txt", "r");

    int op_hand;
    int my_hand;

    int total_score = 0;

    char line[4];
    while(fgets(line, sizeof(line), fptr) != NULL) {
        op_hand = move_as_int(line[0]);
        my_hand = move_as_int(line[2]);
        total_score += scores[op_hand][my_hand];
    }

    printf("%d\n", total_score);
    
    return 0;
}

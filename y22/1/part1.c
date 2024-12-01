#include <stdio.h>
#include <stdlib.h>

int main() {
    FILE *fptr;

    fptr = fopen("input.txt", "r");
    
    // Get size of file (bytes i presume)
    // fseek(fptr, 0, SEEK_END);
    // int fsize = ftell(fptr);
    // fseek(fptr, 0, SEEK_SET);

    // printf("%d\n", fsize);

    int max_calories = 0;

    char line[10];
    int tot_cal = 0;
    while (fgets(line, sizeof(line), fptr) != NULL) {
        if (line[0] == '\n') {
            if (tot_cal > max_calories) {
                max_calories = tot_cal;
            }
            tot_cal = 0;
        } else {
            tot_cal += atoi(line);
        }
    }

    printf("%d\n", max_calories);

    return 0;
}


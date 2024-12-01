#include <stdio.h>
#include <stdlib.h>

void replace_smallest(int arr[3], int val) {
    int min = val;
    int i = -1;
    for (int j = 0; j < 3; j++) {
        if (arr[j] < min) {
            min = arr[j];
            i = j;
        }
    }
    if (i != -1) {
        arr[i] = val;
    }
}

int main() {
    FILE *fptr;

    fptr = fopen("input.txt", "r");
    
    int max_calories[3] = {0, 0, 0};

    char line[10];
    int tot_cal = 0;
    while (fgets(line, sizeof(line), fptr) != NULL) {
        if (line[0] == '\n') {
            replace_smallest(max_calories, tot_cal);
            tot_cal = 0;
        } else {
            tot_cal += atoi(line);
        }
    }

    fclose(fptr);

    int sum = 0;
    printf("Top three:\n");
    for (int i = 0; i < 3; i++) {
        sum += max_calories[i];
        printf("%d\n", max_calories[i]);
    }
    printf("Total: "); 
    printf("%d\n", sum); 

    return 0; }


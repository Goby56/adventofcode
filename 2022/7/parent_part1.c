#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>
#include <string.h>

#define LINE_BUFFER 100

struct Node {
    char *name;
    int size;
    bool is_dir;
    struct Node *parent;
};

char *get_name(char line[LINE_BUFFER]) {
    int start = 0;
    int i;
    for (i = 1; i < LINE_BUFFER; i++) {
        if (line[i-1] == ' ') {
            start = i;
        } if (line[i] == '\n') {
            break;
        }
    }
    char *name = malloc(i - start + 1);
    strncpy(name, line + start, i - start);
    return name;
}

int get_size(char line[LINE_BUFFER]) {
    int i;
    for (i = 0; i < LINE_BUFFER; i++) {
        if (line[i] == ' ') {
            break;
        }
    }
    char *size = malloc(i + 1);
    strncpy(size, line, i);
    return atoi(size);
}

void create_node(struct Node *p_node, char line[LINE_BUFFER], struct Node *parent, bool is_dir) {
    p_node->name = get_name(line);
    if (is_dir) {
        p_node->size = 0;
    } else {
        p_node->size = get_size(line);
    }
    p_node->is_dir = is_dir;
    p_node->parent = parent;
}

struct Node *get_node(struct Node nodes[], int node_count, char *name) {
    printf("test\n");
    printf("looking for %s in: ", name);
    for (int i = 0; i < node_count; i++) {
        printf("%s, ", nodes[i].name);
        if (strcmp(nodes[i].name, name) == 0) {
            printf("\n");
            return &nodes[i];
        }
    }
    return NULL;
}

int main() {
    FILE *fptr = fopen("test.txt", "r");
    char line[LINE_BUFFER];
    int node_count = 1;
    while (fgets(line, sizeof(line), fptr) != NULL) {
        if (line[0] != '$') {
            node_count++;
        }
    }
    fseek(fptr, 0, SEEK_SET);

    struct Node *nodes = malloc(node_count * sizeof(struct Node));
    struct Node *curr_parent = nodes;
    curr_parent->name = "/";
    curr_parent->size = 0;
    curr_parent->is_dir = true;
    int i = 1;
    printf("%s\n", curr_parent->name);
    while (fgets(line, sizeof(line), fptr) != NULL) {
        printf("%s", line);
        switch (line[0]) {
            case '$':
               if (strncmp(line+2, "cd", 2) == 0) {
                   printf("node: %s\n", get_name(line));
                   curr_parent = get_node(nodes, node_count, get_name(line));
               }
            case 'd':
               create_node(&nodes[i], line, curr_parent, true);
            default:
               create_node(&nodes[i], line, curr_parent, false);
        }
    }

    for (int i = 0; i < node_count; i++) {
        printf("%s\n", nodes[i].name);
        free(&nodes[i]);
    }
    free(nodes);
    // Loop through all nodes. If file, increment parent size
    // hmm what to do with dirs? need while loop to enure each
    // dir is updated 
    return 0;
}

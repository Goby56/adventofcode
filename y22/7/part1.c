#include <math.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define LINE_BUFF 100

char *get_name(char line[LINE_BUFF]) {
    int start = 0;
    int i;
    for (i = 1; i < LINE_BUFF; i++) {
        if (line[i-1] == ' ') {
            start = i;
        } 
        if (line[i] == '\n') {
            break;
        }
    }

    char *name = calloc(i - start + 1, sizeof(char));
    strncpy(name, line + start, i - start);
    return name;
}

int get_size(char line[LINE_BUFF]) {
    int i;
    for (i = 0; i < LINE_BUFF; i++) {
        if (line[i] == ' ') {
            break;
        }
    }
    char *size_str = calloc(i + 1, sizeof(char));
    strncpy(size_str, line, i);
    int size = atoi(size_str);
    free(size_str);
    return size;
}

typedef struct Node {
    char *name;
    int size;
    bool is_dir;
    int child_count;
    struct Node **children;
    struct Node *parent;
} Node;

Node *create_node(char line[LINE_BUFF], bool is_dir) {
    Node *node = malloc(sizeof(Node));
    node->name = get_name(line);
    node->size = is_dir ? 0 : get_size(line);
    node->is_dir = is_dir;
    node->child_count = 0;
    node->children = malloc(sizeof(Node*));
    return node;
}

void add_child(Node *node, Node *parent) {
    node->parent = parent;
    parent->child_count++;
    parent->children = 
        realloc(parent->children, parent->child_count * sizeof(Node*));
    parent->children[parent->child_count - 1] = node;
}

Node *find_node(char *name, Node *search_node) {
    if (strcmp(name, "..") == 0) {
        return search_node->parent;
    }
    for (int i = 0; i < search_node->child_count; i++) {
        if (strcmp(search_node->children[i]->name, name) == 0) {
            return *(search_node->children + i);
        }
    }
    exit(134);
}

void free_node(Node *node) {
    printf("freeing %s\n", node->name);
    if (strcmp(node->name, "/") == 0) {
        printf("%s\n", node->name);
    }
    free(node->name);
    free(node->children);
    free(node);
}

void free_tree(Node *root_node) {
    if (!root_node->is_dir) {
        free_node(root_node);
        return;
    }
    for (int i = 0; i < root_node->child_count; i++) {
        free_tree(root_node->children[i]);
    }
    free_node(root_node);
}

void sum_tree(Node *root_node, int *parent_size, int *tot_sum) {
    if (!root_node->is_dir) {
        *parent_size += root_node->size;
        return;
    }
    for (int i = 0; i < root_node->child_count; i++) {
        sum_tree(root_node->children[i], &root_node->size, tot_sum);
    }
    *parent_size += root_node->size;

    if (root_node->size < 100000) {
        *tot_sum += root_node->size;
    }
}

int main() {
    FILE *fptr = fopen("test.txt", "r");
    char line[100];

    fgets(line, sizeof(line), fptr);
    Node root_node = *create_node(line, true);
    Node *curr_parent = &root_node;
    while (fgets(line, sizeof(line), fptr) != NULL) {
        if (line[0] == '$') {
            if (strncmp(line+2, "cd", 2) == 0) {
                char *node_name = get_name(line);
                curr_parent = find_node(node_name, curr_parent);
                free(node_name);
            }
        } else if (line[0] == 'd') {
            add_child(create_node(line, true), curr_parent);
        } else {
            add_child(create_node(line, false), curr_parent);
        }
    }
    fclose(fptr);

    int dir_sum_under_100k = 0;
    sum_tree(&root_node, &root_node.size, &dir_sum_under_100k);
    printf("%d\n", dir_sum_under_100k);

    curr_parent = &root_node;
    free_tree(curr_parent);

    return 0;
}

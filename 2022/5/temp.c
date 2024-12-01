
void move(char **crates, int src, int dst, int amt) {
    for (int i = 0; i < amt; i++) {
        int in_src = 0;
        int in_dst = 0;
        while (1) {
            if (crates[src][in_src] == ' ' &&
                crates[dst][in_dst] == ' ') {
                break;
            }
            if (crates[src][in_src] != ' ') {
                in_src++;
            }
            if (crates[dst][in_dst] != ' ') {
                in_dst++;
            }
        }
        strcpy(&crates[dst][++in_dst], &crates[src][in_src]);
        strcpy(&crates[src][--in_src], " ");
    }
}

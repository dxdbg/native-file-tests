#include <stdio.h>
#include <stdlib.h>

int max(int a, int b);

int main(int argc, char *argv[]) {
    int maxNum = max(3, 4);

    printf("Max: %d\n", maxNum);

    return maxNum != 4;
}

int max(int a, int b) { 
    return ( a >= b ) ? a : b;
}

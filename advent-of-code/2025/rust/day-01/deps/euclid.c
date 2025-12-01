// gcc -c euclid.c -o euclid.o

#include <stdlib.h>

int euclidean_remainder(int a, int b) {
    // assert(b != 0);
    int r = a % b;
    return r >= 0 ? r : r + abs(b);
}   
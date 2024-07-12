#include <stdio.h>

int test_data = 5;

extern int test();

int main() {
    printf("func() => %d", test());
    return 0;
}
#include "../../common/include/kernel/syscall-definitions.h"
#include "sys/syscall.h"
#include "unistd.h"
#include "stdio.h"
#include "string.h"
#include "stdlib.h"
#include "nonstd.h"


int main() {
    size_t arg = 101;
    printf("adding 1 to %zu\n", arg);
    size_t result = __syscall(sc_addone, (size_t) arg, 0, 0, 0, 0);
    printf("1 + %zu = %zu\n", arg, result);
    return 0;
}
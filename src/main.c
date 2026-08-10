#include "runtime.h"

#include <stdio.h>

int main(void)
{
    ChaosRuntime runtime;

    chaos_runtime_init(&runtime);

    printf("Chaos runtime initialized.\n");
    printf("Clock mode: synchronous\n");
    printf("Runtime state: running\n");

    chaos_runtime_free(&runtime);

    printf("Chaos runtime terminated.\n");

    return 0;
}
#include <stdio.h>
#include "chaos.h"
#include "runtime.h"

void chaos_start(void)
{
    printf("Chaos is alive.\n");
    runtime_start();
}

int main(void)
{
    chaos_start();
    return 0;
}
#include <stdio.h>
#include "resolver.h"

int main(void)
{
    Resolution result = resolve("hello");

    for (int i = 0; i < result.count; i++)
    {
        printf("%s\n", result.conditions[i]);
    }

    return 0;
}
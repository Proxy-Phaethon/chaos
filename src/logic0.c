#include <stdio.h>
#include <string.h>
#include "logic0.h"

void run_logic0(const char *question, char *value, int size)
{
    printf("%s\n", question);

    if (fgets(value, size, stdin) != NULL)
    {
        value[strcspn(value, "\n")] = '\0';
    }
}
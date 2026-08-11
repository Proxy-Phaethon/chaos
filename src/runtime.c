#include <stdio.h>
#include <string.h>
#include "runtime.h"
#include "builtins.h"

void run_builtins(char builtins[][64], int count, char *value, int size)
{
    for (int i = 0; i < count; i++)
    {
        BuiltinFunction function = find_builtin(builtins[i]);

        if (function == NULL)
        {
            printf("Unknown built-in: %s\n", builtins[i]);
            continue;
        }

        char *output = function(value);

        if (output != NULL)
        {
            strncpy(value, output, size - 1);
            value[size - 1] = '\0';
        }
    }
}
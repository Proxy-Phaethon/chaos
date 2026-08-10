#include <stdio.h>
#include <string.h>
#include <stddef.h>
#include "builtins.h"

//write the built-ins below this line.

const char *function_b(const char *input)
{
    static char result[256];

    size_t length = strlen(input);

    if (length > 0 && input[length - 1] == '\n')
    {
        length--;
    }

    int is_word = 1;

    for (size_t i = 0; i < length; i++)
    {
        if ((input[i] < 'A' || input[i] > 'Z') &&
            (input[i] < 'a' || input[i] > 'z'))
        {
            is_word = 0;
            break;
        }
    }

    snprintf(result, sizeof(result), "%.*s%c",
             (int)length, input, is_word ? 'b' : 'c');

    return result;
}

Builtin builtins[] =
{
    // here you can add the names of more built-ins to the registry.
    {"function_b", function_b},
    {NULL, NULL}
};

BuiltinFunction find_builtin(const char *name)
{
    int i = 0;

    while (builtins[i].name != NULL)
    {
        if (strcmp(name, builtins[i].name) == 0)
        {
            return builtins[i].function;
        }

        i++;
    }

    return NULL;
}
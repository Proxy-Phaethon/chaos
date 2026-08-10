#include <ctype.h>
#include <string.h>
#include "resolver.h"

Resolution resolve(const char *output)
{
    Resolution result;
    result.count = 0;

    int is_empty = 1;
    int is_number = 1;
    int is_word = 1;

    for (int i = 0; output[i] != '\0'; i++)
    {
        is_empty = 0;

        if (!isdigit((unsigned char)output[i]))
        {
            is_number = 0;
        }

        if (!isalpha((unsigned char)output[i]))
        {
            is_word = 0;
        }
    }

    if (is_empty)
    {
        result.conditions[result.count++] = "empty";
    }

    if (is_number)
    {
        result.conditions[result.count++] = "number";
    }

    if (is_word)
    {
        result.conditions[result.count++] = "word";
    }

    return result;
}
#include <ctype.h>
#include <string.h>
#include "conditions.h"

static int is_word(const char *output)
{
    if (output[0] == '\0')
    {
        return 0;
    }

    for (int i = 0; output[i] != '\0'; i++)
    {
        if (!isalpha((unsigned char)output[i]))
        {
            return 0;
        }
    }

    return 1;
}

static int is_number(const char *output)
{
    if (output[0] == '\0')
    {
        return 0;
    }

    for (int i = 0; output[i] != '\0'; i++)
    {
        if (!isdigit((unsigned char)output[i]))
        {
            return 0;
        }
    }

    return 1;
}

static int is_empty(const char *output)
{
    return output[0] == '\0';
}

// use format 
// static int is_something(const char *output)
// {
//     ...
// }
// to add conditions.

Condition conditions[] =
{
    {"word", is_word},
    {"number", is_number},
    {"empty", is_empty},
    // {"something", is_something},
    {NULL, NULL}
};

int is_condition(const char *name)
{
    int i = 0;

    while (conditions[i].name != NULL)
    {
        if (strcmp(name, conditions[i].name) == 0)
        {
            return 1;
        }

        i++;
    }

    return 0;
}
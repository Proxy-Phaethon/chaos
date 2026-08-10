#include <stdio.h>
#include <string.h>
#include <ctype.h>
#include "builtins.h"

static char metastable_output[256];
static char sanitizer_output[256];

static char *metastable(const char *input)
{
    snprintf(metastable_output, sizeof(metastable_output), "%s", input);

    return metastable_output;
}

static char *sanitizer(const char *input)
{
    int j = 0;

    for (int i = 0; input[i] != '\0' && j < 255; i++)
    {
        if (!isspace((unsigned char)input[i]))
        {
            sanitizer_output[j++] = input[i];
        }
    }

    sanitizer_output[j] = '\0';

    return sanitizer_output;
}

//add builtin details here
//static char *validator(const char *input)
//{
//    Implement the validator function here}

Builtin builtins[] =
{
    {"metastable", metastable},
    {"sanitizer", sanitizer},
    // add builtin here {"something", something},
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
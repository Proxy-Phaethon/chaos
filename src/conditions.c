#include <string.h>
#include "conditions.h"

Condition conditions[] =
{
    {"word"},
    {"number"},
    {"valid"},
    {"invalid"},
    {"true"},
    {"false"},
    {"empty"},
    {"exists"},
    {NULL}
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
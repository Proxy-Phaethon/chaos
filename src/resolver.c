#include "resolver.h"
#include "conditions.h"
#include <stddef.h>

Resolution resolve(const char *output)
{
    Resolution result;
    result.count = 0;

    int i = 0;

    while (conditions[i].name != NULL)
    {
        if (conditions[i].function(output))
        {
            result.conditions[result.count++] = conditions[i].name;
        }

        i++;
    }

    return result;
}
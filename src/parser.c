#include <stdio.h>
#include <string.h>
#include "logic0.h"

void parse_chaos(const char *line)
{
    if (strncmp(line, "logic0", 6) == 0)
    {
        const char *start = strchr(line, '(');
        const char *end = strrchr(line, ')');

        if (start != NULL && end != NULL && end > start)
{
            char question[256];

            snprintf(question, sizeof(question), "%.*s",
                (int)(end - start - 3), start + 2);

            logic0(question);
}
    }
}
#include <stdio.h>
#include <string.h>
#include "parser.h"
#include "logic0.h"
#include "builtins.h"

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
    else
    {
        BuiltinFunction builtin = find_builtin(line);

        if (builtin != NULL)
        {
            const char *result = builtin(logic0_value());

            printf("%s\n", result);
        }
    }
}
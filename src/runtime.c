#include <stdio.h>
#include "runtime.h"
#include "logic0.h"
#include "builtins.h"

void execute_line(const char *line)
{
    BuiltinFunction builtin = find_builtin(line);

    if (builtin != NULL)
    {
        const char *result = builtin(logic0_value());

        printf("%s\n", result);
    }
}
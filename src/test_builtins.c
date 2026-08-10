#include <stdio.h>
#include "builtins.h"

int main(void)
{
    BuiltinFunction function;

    function = find_builtin("metastable");

    if (function != NULL)
    {
        printf("%s\n", function("hello world"));
    }

    function = find_builtin("sanitizer");

    if (function != NULL)
    {
        printf("%s\n", function(" hello world "));
    }

    return 0;
}
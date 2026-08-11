#include <stdio.h>
#include "parser.h"
#include "runtime.h"
#include "logic0.h"

int main(int argc, char *argv[])
{
    FILE *file;
    char line[256];
    char question[256];
    char builtins[10][64];
    char value[256] = "";

    if (argc < 2)
    {
        printf("Usage: ./chaos <file.chaos>\n");
        return 1;
    }

    file = fopen(argv[1], "r");

    if (file == NULL)
    {
        printf("Could not open file.\n");
        return 1;
    }

    while (fgets(line, sizeof(line), file) != NULL)
    {
        if (parse_chaos(line, question, sizeof(question)))
        {
            run_logic0(question, value, sizeof(value));
        }

        int count = parse_call(line, builtins, 10);

        if (count > 0)
        {
            run_builtins(builtins, count, value, sizeof(value));

            printf("Result: %s\n", value);
        }
    }

    fclose(file);

    return 0;
}
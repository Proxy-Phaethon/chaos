#include <stdio.h>
#include "parser.h"
#include "runtime.h"

int main(int argc, char *argv[])
{
    FILE *file;
    char line[256];
    char value[256] = "";
    Statement statements[256];
    int statement_count = 0;

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
        if (statement_count >= 256)
        {
            printf("Too many statements.\n");
            break;
        }

        if (parse_line(line, &statements[statement_count]))
        {
            statement_count++;
        }
    }

    fclose(file);

    run_program(statements, statement_count, value, sizeof(value));

    return 0;
}
#include <stdio.h>
#include "parser.h"
#include "runtime.h"

int main(int argc, char *argv[])
{
    FILE *file;
    char line[256];
    char value[256] = "";
    Statement statement;

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
        if (parse_line(line, &statement))
        {
            run_statement(&statement, value, sizeof(value));
        }
    }

    fclose(file);

    return 0;
}
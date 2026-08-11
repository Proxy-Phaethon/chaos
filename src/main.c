#include <stdio.h>
#include "parser.h"

int main(int argc, char *argv[])
{
    FILE *file;
    char line[256];
    char question[256];
    char builtins[10][64];

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
            printf("%s\n", question);
        }

        int count = parse_call(line, builtins, 10);

        if (count > 0)
        {
            for (int i = 0; i < count; i++)
            {
                printf("Built-in: %s\n", builtins[i]);
            }
        }
    }

    fclose(file);

    return 0;
}
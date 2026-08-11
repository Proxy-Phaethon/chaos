#include <stdio.h>
#include <string.h>
#include "parser.h"

int main(int argc, char *argv[])
{
    FILE *file;
    char line[256];
    char question[256];

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
    }

    fclose(file);

    return 0;
}
#include <stdio.h>
#include "parser.h"

int main(void)
{
    FILE *file;
    char line[256];

    file = fopen("test.chaos", "r");

    if (file == NULL)
    {
        printf("Could not open test.chaos\n");
        return 1;
    }

    while (fgets(line, sizeof(line), file) != NULL)
    {
        parse_chaos(line);
    }

    fclose(file);

    return 0;
}
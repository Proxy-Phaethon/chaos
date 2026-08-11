#include <string.h>
#include "parser.h"

int parse_chaos(const char *line, char *question, int size)
{
    const char *start;
    const char *end;
    int length;

    if (strncmp(line, "logic0 ('", 9) != 0)
    {
        return 0;
    }

    start = line + 9;
    end = strrchr(start, '\'');

    if (end == NULL)
    {
        return 0;
    }

    length = end - start;

    if (length >= size)
    {
        return 0;
    }

    strncpy(question, start, length);
    question[length] = '\0';

    return 1;
}
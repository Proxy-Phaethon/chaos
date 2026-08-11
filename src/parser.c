#include <string.h>
#include "parser.h"

int parse_line(const char *line, Statement *statement)
{
    char cleaned_line[256];

    strncpy(cleaned_line, line, sizeof(cleaned_line) - 1);
    cleaned_line[sizeof(cleaned_line) - 1] = '\0';

    cleaned_line[strcspn(cleaned_line, "\r\n")] = '\0';

    line = cleaned_line;
    while (*line == ' ' || *line == '\t')
    {
        line++;
    }

    statement->type = STATEMENT_NONE;
    statement->value[0] = '\0';

    if (strncmp(line, "logic0 ('", 9) == 0)
    {
        const char *start = line + 9;
        const char *end = strrchr(start, '\'');

        if (end == NULL)
        {
            return 0;
        }

        int length = end - start;

        if (length >= 256)
        {
            return 0;
        }

        strncpy(statement->value, start, length);
        statement->value[length] = '\0';
        statement->type = STATEMENT_LOGIC0;

        return 1;
    }

    if (strncmp(line, "call ", 5) == 0)
    {
        strncpy(statement->value, line + 5, 255);
        statement->value[255] = '\0';

        statement->type = STATEMENT_CALL;
        return 1;
    }

    if (strncmp(line, "if ", 3) == 0)
    {
        strncpy(statement->value, line + 3, 255);
        statement->value[255] = '\0';

        statement->type = STATEMENT_IF;
        return 1;
    }

    if (strncmp(line, "else if ", 8) == 0)
    {
        strncpy(statement->value, line + 8, 255);
        statement->value[255] = '\0';

        statement->type = STATEMENT_ELSE_IF;
        return 1;
    }

    if (strncmp(line, "else", 4) == 0)
    {
        statement->type = STATEMENT_ELSE;
        return 1;
    }

    if (strncmp(line, "action ('", 9) == 0)
    {
        const char *start = line + 9;
        const char *end = strrchr(start, '\'');

        if (end == NULL)
        {
            return 0;
        }

        int length = end - start;

        if (length >= 256)
        {
            return 0;
        }

        strncpy(statement->value, start, length);
        statement->value[length] = '\0';
        statement->type = STATEMENT_ACTION;

        return 1;
    }

    if (strncmp(line, "terminate", 9) == 0)
    {
        statement->type = STATEMENT_TERMINATE;
        return 1;
    }

    return 0;
}
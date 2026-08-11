#ifndef PARSER_H
#define PARSER_H

typedef enum
{
    STATEMENT_NONE,
    STATEMENT_LOGIC0,
    STATEMENT_CALL,
    STATEMENT_IF,
    STATEMENT_ELSE_IF,
    STATEMENT_ELSE,
    STATEMENT_ACTION,
    STATEMENT_TERMINATE
} StatementType;

typedef struct
{
    StatementType type;
    char value[256];
} Statement;

int parse_line(const char *line, Statement *statement);

#endif
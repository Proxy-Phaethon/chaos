#include <stdio.h>
#include "runtime.h"
#include "logic0.h"

void run_statement(Statement *statement, char *value, int size)
{
    switch (statement->type)
    {
        case STATEMENT_LOGIC0:
            run_logic0(statement->value, value, size);
            break;

        default:
            break;
    }
}
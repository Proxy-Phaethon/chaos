#include <stdio.h>
#include <string.h>

#include "runtime.h"
#include "logic0.h"
#include "builtins.h"
#include "resolver.h"
#include "contracts.h"

int condition_matches(const char *value, const char *condition)
{
    Resolution result = resolve(value);

    for (int i = 0; i < result.count; i++)
    {
        if (strcmp(result.conditions[i], condition) == 0)
        {
            return 1;
        }
    }

    return 0;
}

void run_statement(Statement *statement, char *value, int size)
{
    switch (statement->type)
    {
        case STATEMENT_LOGIC0:
            run_logic0(statement->value, value, size);
            break;

        case STATEMENT_CALL:
        {
            char builtin_names[10][64];
            char buffer[256];
            char *token;
            int count = 0;

            strncpy(buffer, statement->value, sizeof(buffer) - 1);
            buffer[sizeof(buffer) - 1] = '\0';

            token = strtok(buffer, ",\n");

            while (token != NULL && count < 10)
            {
                while (*token == ' ')
                {
                    token++;
                }

                strncpy(builtin_names[count], token, 63);
                builtin_names[count][63] = '\0';

                count++;
                token = strtok(NULL, ",\n");
            }

            for (int i = 0; i < count; i++)
            {
                BuiltinFunction function = find_builtin(builtin_names[i]);

                if (function == NULL)
                {
                    printf("Unknown built-in: %s\n", builtin_names[i]);
                    continue;
                }

                char *output = function(value);

                if (output != NULL)
                {
                    strncpy(value, output, size - 1);
                    value[size - 1] = '\0';
                }
            }

            break;
        }

        case STATEMENT_IF:
            break;

        case STATEMENT_ACTION:
        {
            ContractFunction function = find_contract(statement->value);

            if (function == NULL)
            {
                printf("Unknown contract: %s\n", statement->value);
                break;
            }

            function(value);
            break;
        }

        case STATEMENT_TERMINATE:
            printf("Action terminated.\n");
            break;

        default:
            break;
    }
}

void run_program(Statement *statements, int count, char *value, int size)
{
    int i = 0;

    while (i < count)
    {
        if (statements[i].type == STATEMENT_IF)
        {
            int branch_matched = 0;

            if (condition_matches(value, statements[i].value))
            {
                branch_matched = 1;

                if (i + 1 < count &&
                    (statements[i + 1].type == STATEMENT_ACTION ||
                     statements[i + 1].type == STATEMENT_TERMINATE))
                {
                    run_statement(&statements[i + 1], value, size);
                }

                i += 2;
            }
            else
            {
                i += 2;

                while (i < count &&
                       statements[i].type == STATEMENT_ELSE_IF)
                {
                    if (condition_matches(value, statements[i].value))
                    {
                        branch_matched = 1;

                        if (i + 1 < count &&
                            (statements[i + 1].type == STATEMENT_ACTION ||
                             statements[i + 1].type == STATEMENT_TERMINATE))
                        {
                            run_statement(&statements[i + 1], value, size);
                        }

                        i += 2;
                        break;
                    }

                    i += 2;
                }
            }

            if (!branch_matched &&
                i < count &&
                statements[i].type == STATEMENT_ELSE)
            {
                if (i + 1 < count &&
                    (statements[i + 1].type == STATEMENT_ACTION ||
                     statements[i + 1].type == STATEMENT_TERMINATE))
                {
                    run_statement(&statements[i + 1], value, size);
                }

                i += 2;
            }

            while (branch_matched &&
                   i < count &&
                   (statements[i].type == STATEMENT_ELSE_IF ||
                    statements[i].type == STATEMENT_ELSE))
            {
                i += 2;
            }

            continue;
        }

        run_statement(&statements[i], value, size);
        i++;
    }
}
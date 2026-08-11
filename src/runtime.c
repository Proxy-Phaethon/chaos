#include <stdio.h>
#include <string.h>

#include "runtime.h"
#include "logic0.h"
#include "builtins.h"
#include "resolver.h"

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
            if (condition_matches(value, statement->value))
            {
                printf("Condition matched: %s\n", statement->value);
            }
            else
            {
                printf("Condition did not match: %s\n", statement->value);
            }
            break;

        default:
            break;
    }
}
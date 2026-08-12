#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "contracts.h"

static void contract_print(const char *input)
{
    printf("%s\n", input);
}

static void contract_terminate(const char *input)
{
    (void)input;
    exit(0);
}

static void contract_double(const char *input)
{
    int value = atoi(input);

    printf("%d\n", value * 2);
}

static void contract_clear(const char *input)
{
    (void)input;

    printf("\033[2J\033[H");
}

static void contract_increment(const char *input)
{
    int value = atoi(input);

    printf("%d\n", value + 1);
}

static void contract_decrement(const char *input)
{
    int value = atoi(input);

    printf("%d\n", value - 1);
}

static void contract_reset(const char *input)
{
    (void)input;

    printf("0\n");
}

static void contract_triple(const char *input)
{
    int value = atoi(input);

    printf("%d\n", value * 3);
}

static void contract_square(const char *input)
{
    int value = atoi(input);

    printf("%d\n", value * value);
}

static void contract_halve(const char *input)
{
    int value = atoi(input);

    printf("%d\n", value / 2);
}

static void contract_negate(const char *input)
{
    int value = atoi(input);

    printf("%d\n", -value);
}

static void contract_absolute(const char *input)
{
    int value = atoi(input);

    printf("%d\n", abs(value));
}

static void contract_log(const char *input)
{
    printf("%s\n", input);
}

//add new contract here

Contract contracts[] =
{
    {"print", contract_print},
    {"terminate", contract_terminate},
    {"double", contract_double},
    {"clear", contract_clear},
    {"increment", contract_increment},
    {"decrement", contract_decrement},
    {"reset", contract_reset},
    {"triple", contract_triple},
    {"square", contract_square},
    {"halve", contract_halve},
    {"negate", contract_negate},
    {"absolute", contract_absolute},
    {"log", contract_log},
    //add new contract here
    {NULL, NULL}
};

ContractFunction find_contract(const char *name)
{
    int i = 0;

    while (contracts[i].name != NULL)
    {
        if (strcmp(name, contracts[i].name) == 0)
        {
            return contracts[i].function;
        }

        i++;
    }

    return NULL;
}
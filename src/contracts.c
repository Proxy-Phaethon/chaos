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

//add new contract here

Contract contracts[] =
{
    {"print", contract_print},
    {"terminate", contract_terminate},
    {"double", contract_double},
    {"clear", contract_clear},
    {"increment", contract_increment},
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
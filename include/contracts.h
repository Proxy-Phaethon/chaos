#ifndef CONTRACTS_H
#define CONTRACTS_H

typedef void (*ContractFunction)(const char *input);

typedef struct
{
    const char *name;
    ContractFunction function;
} Contract;

extern Contract contracts[];

ContractFunction find_contract(const char *name);

#endif
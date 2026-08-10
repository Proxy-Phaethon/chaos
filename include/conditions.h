#ifndef CONDITIONS_H
#define CONDITIONS_H

typedef int (*ConditionFunction)(const char *output);

typedef struct
{
    const char *name;
    ConditionFunction function;
} Condition;

extern Condition conditions[];

int is_condition(const char *name);

#endif
#ifndef CONDITIONS_H
#define CONDITIONS_H

typedef struct
{
    const char *name;
} Condition;

extern Condition conditions[];

int is_condition(const char *name);

#endif
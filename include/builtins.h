#ifndef BUILTINS_H
#define BUILTINS_H

typedef const char *(*BuiltinFunction)(const char *input);

typedef struct
{
    const char *name;
    BuiltinFunction function;
} Builtin;

const char *function_b(const char *input);

extern Builtin builtins[];

BuiltinFunction find_builtin(const char *name);

#endif
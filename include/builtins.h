#ifndef BUILTINS_H
#define BUILTINS_H

typedef char *(*BuiltinFunction)(const char *input);

typedef struct
{
    const char *name;
    BuiltinFunction function;
} Builtin;

extern Builtin builtins[];

BuiltinFunction find_builtin(const char *name);

#endif
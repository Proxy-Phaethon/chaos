#ifndef CHAOS_BUILTIN_H
#define CHAOS_BUILTIN_H

#include <stddef.h>

typedef enum {
    CHAOS_VALUE_NONE,
    CHAOS_VALUE_TEXT,
    CHAOS_VALUE_NUMBER,
    CHAOS_VALUE_BOOLEAN
} ChaosValueType;

typedef struct {
    ChaosValueType type;

    union {
        char *text;
        double number;
        int boolean;
    } data;
} ChaosValue;

typedef struct {
    ChaosValue value;

    char **conditions;
    size_t condition_count;
} ChaosBuiltinResult;

typedef struct ChaosRuntime ChaosRuntime;

typedef ChaosBuiltinResult (*ChaosBuiltinFunction)(
    ChaosRuntime *runtime,
    const ChaosValue *input
);

typedef struct {
    char *name;
    ChaosBuiltinFunction function;
} ChaosBuiltin;

typedef struct {
    ChaosBuiltin *items;
    size_t count;
    size_t capacity;
} ChaosBuiltinRegistry;

void chaos_builtins_init(
    ChaosBuiltinRegistry *registry
);

int chaos_builtin_register(
    ChaosBuiltinRegistry *registry,
    const char *name,
    ChaosBuiltinFunction function
);

ChaosBuiltinFunction chaos_builtin_find(
    const ChaosBuiltinRegistry *registry,
    const char *name
);

void chaos_builtins_free(
    ChaosBuiltinRegistry *registry
);

#endif
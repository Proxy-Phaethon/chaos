#ifndef CHAOS_RUNTIME_H
#define CHAOS_RUNTIME_H

#include "condition.h"
#include "builtin.h"
#include "action.h"

typedef enum {
    CHAOS_CLOCK_SYNC,
    CHAOS_CLOCK_ASYNC
} ChaosClockMode;

typedef struct ChaosRuntime {
    ChaosConditionRegistry conditions;
    ChaosBuiltinRegistry builtins;
    ChaosActionRegistry actions;

    ChaosClockMode clock_mode;

    int running;
} ChaosRuntime;

void chaos_runtime_init(
    ChaosRuntime *runtime
);

void chaos_runtime_free(
    ChaosRuntime *runtime
);

ChaosBuiltinResult chaos_runtime_call_builtin(
    ChaosRuntime *runtime,
    const char *name,
    const ChaosValue *input
);

int chaos_runtime_execute_action(
    ChaosRuntime *runtime,
    const char *name
);

void chaos_runtime_terminate(
    ChaosRuntime *runtime
);

#endif
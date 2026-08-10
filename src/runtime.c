#include "runtime.h"

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

void chaos_runtime_init(ChaosRuntime *runtime)
{
    if (runtime == NULL) {
        return;
    }

    chaos_conditions_init(&runtime->conditions);
    chaos_builtins_init(&runtime->builtins);
    chaos_actions_init(&runtime->actions);

    runtime->clock_mode = CHAOS_CLOCK_SYNC;
    runtime->running = 1;
}

void chaos_runtime_free(ChaosRuntime *runtime)
{
    if (runtime == NULL) {
        return;
    }

    chaos_conditions_free(&runtime->conditions);
    chaos_builtins_free(&runtime->builtins);
    chaos_actions_free(&runtime->actions);

    runtime->running = 0;
}

ChaosBuiltinResult chaos_runtime_call_builtin(
    ChaosRuntime *runtime,
    const char *name,
    const ChaosValue *input
)
{
    ChaosBuiltinResult result = {0};

    if (runtime == NULL || name == NULL) {
        return result;
    }

    ChaosBuiltinFunction builtin =
        chaos_builtin_find(&runtime->builtins, name);

    if (builtin == NULL) {
        fprintf(
            stderr,
            "Chaos runtime: unknown built-in '%s'\n",
            name
        );

        return result;
    }

    result = builtin(runtime, input);

    return result;
}

int chaos_runtime_execute_action(
    ChaosRuntime *runtime,
    const char *name
)
{
    if (runtime == NULL || name == NULL) {
        return 0;
    }

    ChaosAction *action =
        chaos_action_find(&runtime->actions, name);

    if (action == NULL) {
        fprintf(
            stderr,
            "Chaos runtime: unknown action '%s'\n",
            name
        );

        return 0;
    }

    /*
     * Contract execution will be implemented here.
     *
     * The runtime owns execution.
     */
    for (size_t i = 0; i < action->contract_count; i++) {
        printf(
            "Executing contract: %s\n",
            action->contracts[i].name
        );
    }

    return 1;
}

void chaos_runtime_terminate(
    ChaosRuntime *runtime
)
{
    if (runtime == NULL) {
        return;
    }

    runtime->running = 0;
}
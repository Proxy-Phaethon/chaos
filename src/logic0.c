#include "runtime.h"

#include <stdio.h>
#include <string.h>

typedef struct {
    const char *condition;
    const char *action;
} LogicBranch;

typedef struct {
    LogicBranch *branches;
    size_t branch_count;

    const char *else_action;
} Logic0Program;

static int result_has_condition(
    const ChaosBuiltinResult *result,
    const char *condition
)
{
    if (result == NULL || condition == NULL) {
        return 0;
    }

    for (size_t i = 0; i < result->condition_count; i++) {
        if (strcmp(result->conditions[i], condition) == 0) {
            return 1;
        }
    }

    return 0;
}

int chaos_logic0_run(
    ChaosRuntime *runtime,
    const char *question,
    const char **called_builtins,
    size_t builtin_count,
    const Logic0Program *program
)
{
    if (
        runtime == NULL ||
        question == NULL ||
        called_builtins == NULL ||
        program == NULL
    ) {
        return 0;
    }

    printf("%s\n> ", question);

    char input_buffer[1024];

    if (fgets(input_buffer, sizeof(input_buffer), stdin) == NULL) {
        return 0;
    }

    ChaosValue input = {
        .type = CHAOS_VALUE_TEXT,
        .data.text = input_buffer
    };

    /*
     * Built-ins receive the input.
     *
     * Their results are then inspected by logic0.
     */
    for (size_t i = 0; i < builtin_count; i++) {

        ChaosBuiltinResult result =
            chaos_runtime_call_builtin(
                runtime,
                called_builtins[i],
                &input
            );

        /*
         * Conditions are already attached to the
         * result by the runtime/built-in pipeline.
         *
         * logic0 only asks whether a branch matches.
         */
        for (size_t j = 0; j < program->branch_count; j++) {

            if (
                result_has_condition(
                    &result,
                    program->branches[j].condition
                )
            ) {
                return chaos_runtime_execute_action(
                    runtime,
                    program->branches[j].action
                );
            }
        }
    }

    /*
     * Nothing matched.
     *
     * The else branch is simply another runtime action.
     */
    if (program->else_action != NULL) {
        return chaos_runtime_execute_action(
            runtime,
            program->else_action
        );
    }

    return 1;
}
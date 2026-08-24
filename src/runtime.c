#include "runtime.h"

#include <stdio.h>
#include <stdlib.h>
#include <string.h>


/*
 * Convert an AST data type into a runtime data type.
 */
static RuntimeValueType runtime_data_type(
    const char *type)
{
    if (type == NULL) {
        return RUNTIME_VALUE_STRING;
    }

    if (strcmp(type, "list") == 0) {
        return RUNTIME_VALUE_LIST;
    }

    if (strcmp(type, "queue") == 0) {
        return RUNTIME_VALUE_QUEUE;
    }

    if (strcmp(type, "stack") == 0) {
        return RUNTIME_VALUE_STACK;
    }

    if (strcmp(type, "branch") == 0) {
        return RUNTIME_VALUE_BRANCH;
    }

    return RUNTIME_VALUE_STRING;
}


/*
 * Determine the runtime type of a scalar state.
 *
 * This is intentionally simple for now.
 * The expression system will eventually give Chaos
 * a proper type system.
 */
static RuntimeValueType runtime_scalar_type(
    const char *value)
{
    if (value == NULL) {
        return RUNTIME_VALUE_STRING;
    }

    /*
     * A very basic number check.
     */
    char *end = NULL;

    strtod(value, &end);

    if (end != value && *end == '\0') {
        return RUNTIME_VALUE_NUMBER;
    }

    return RUNTIME_VALUE_STRING;
}


/*
 * Create the runtime.
 */
Runtime *runtime_create(void)
{
    Runtime *runtime = calloc(
        1,
        sizeof(Runtime)
    );

    if (runtime == NULL) {
        return NULL;
    }

    runtime->states =
        runtime_state_store_create();

    if (runtime->states == NULL) {
        free(runtime);
        return NULL;
    }

    return runtime;
}


/*
 * Destroy the runtime.
 */
void runtime_free(
    Runtime *runtime)
{
    if (runtime == NULL) {
        return;
    }

    runtime_state_store_free(
        runtime->states
    );

    free(runtime);
}


/*
 * Execute a REGISTER node.
 *
 * Example:
 *
 * register ('experiment'):
 *     state: x = 3,
 *     state: things, list = {'a', 'b'};
 */
int runtime_execute_register(
    Runtime *runtime,
    const ASTNode *register_node)
{
    if (runtime == NULL ||
        register_node == NULL) {

        return 0;
    }

    for (size_t i = 0;
         i < register_node->child_count;
         i++) {

        const ASTNode *state =
            register_node->children[i];

        if (state->type !=
            AST_STATE_DECLARATION) {

            continue;
        }

        const char *name = NULL;
        const char *value = NULL;
        const char *data_type = NULL;

        /*
         * Extract the state declaration.
         */
        for (size_t j = 0;
             j < state->child_count;
             j++) {

            const ASTNode *child =
                state->children[j];

            switch (child->type) {

                case AST_STATE_NAME:
                    name = child->value;
                    break;

                case AST_STATE_VALUE:
                    value = child->value;
                    break;

                case AST_DATA_TYPE:
                    data_type = child->value;
                    break;

                default:
                    break;
            }
        }

        if (name == NULL) {
            fprintf(
                stderr,
                "Runtime error: state has no name\n"
            );

            return 0;
        }

        RuntimeValueType type;

        if (data_type != NULL) {
            type = runtime_data_type(
                data_type
            );
        }
        else {
            type = runtime_scalar_type(
                value
            );
        }

        RuntimeState *runtime_state =
            runtime_state_create(
                name,
                type,
                value
            );

        if (runtime_state == NULL) {
            fprintf(
                stderr,
                "Runtime error: could not create state '%s'\n",
                name
            );

            return 0;
        }

        /*
         * Initial collection contents are currently
         * stored in the AST as one comma-separated value.
         *
         * The parser will eventually give us proper
         * AST_DATA_ITEMS nodes. For now, populate the
         * collection by splitting the stored value.
         */
        if (data_type != NULL &&
            value != NULL &&
            value[0] != '\0') {

            char *contents = malloc(
                strlen(value) + 1
            );

            if (contents == NULL) {
                free(runtime_state->name);
                runtime_value_free_contents(
                    &runtime_state->value
                );
                free(runtime_state);

                return 0;
            }

            strcpy(contents, value);

            char *item = strtok(
                contents,
                ","
            );

            while (item != NULL) {

                while (*item == ' ') {
                    item++;
                }

                /*
                 * Remove surrounding single quotes.
                 */
                size_t length =
                    strlen(item);

                if (length >= 2 &&
                    item[0] == '\'' &&
                    item[length - 1] == '\'') {

                    item[length - 1] = '\0';
                    item++;
                }

                if (!runtime_state_push(
                        runtime_state,
                        item)) {

                    free(contents);
                    free(runtime_state->name);

                    runtime_value_free_contents(
                        &runtime_state->value
                    );

                    free(runtime_state);

                    return 0;
                }

                item = strtok(NULL, ",");
            }

            free(contents);
        }

        runtime_state_store_add(
            runtime->states,
            runtime_state
        );
    }

    return 1;
}


/*
 * Execute a PUSH/POP state operation.
 */
int runtime_execute_state_operation(
    Runtime *runtime,
    const ASTNode *operation)
{
    if (runtime == NULL ||
        operation == NULL) {

        return 0;
    }

    if (operation->value == NULL) {
        return 0;
    }

    RuntimeState *state =
        runtime_state_find(
            runtime->states,
            operation->value
        );

    if (state == NULL) {
        fprintf(
            stderr,
            "Runtime error: unknown state '%s'\n",
            operation->value
        );

        return 0;
    }

    if (operation->child_count == 0) {
        return 0;
    }

    const ASTNode *action =
        operation->children[0];

    /*
     * PUSH
     */
    if (action->type == AST_PUSH) {

        if (action->child_count == 0) {
            return 0;
        }

        const ASTNode *value =
            action->children[0];

        if (value->value == NULL) {
            return 0;
        }

        if (!runtime_state_push(
                state,
                value->value)) {

            fprintf(
                stderr,
                "Runtime error: could not push into state '%s'\n",
                state->name
            );

            return 0;
        }

        return 1;
    }

    /*
     * POP
     */
    if (action->type == AST_POP) {

        char *value =
            runtime_state_pop(state);

        if (value == NULL) {
            fprintf(
                stderr,
                "Runtime error: cannot pop empty state '%s'\n",
                state->name
            );

            return 0;
        }

        printf(
            "POP %s: %s\n",
            state->name,
            value
        );

        free(value);

        return 1;
    }

    return 0;
}


/*
 * Execute a CONSTANT.
 *
 * At this stage constants are simply recognized.
 * Expression evaluation comes later.
 */
int runtime_execute_constant(
    Runtime *runtime,
    const ASTNode *constant)
{
    (void)runtime;

    if (constant == NULL) {
        return 0;
    }

    if (constant->child_count == 0) {
        return 0;
    }

    const ASTNode *value =
        constant->children[0];

    printf(
        "CONSTANT: %s\n",
        value->value != NULL
        ? value->value
        : ""
    );

    return 1;
}


/*
 * Execute a LOGIC block.
 */
int runtime_execute_logic(
    Runtime *runtime,
    const ASTNode *logic_node)
{
    if (runtime == NULL ||
        logic_node == NULL) {

        return 0;
    }

    for (size_t i = 0;
         i < logic_node->child_count;
         i++) {

        const ASTNode *child =
            logic_node->children[i];

        switch (child->type) {

            case AST_CONSTANT:
                if (!runtime_execute_constant(
                        runtime,
                        child)) {

                    return 0;
                }
                break;

case AST_LIST:
case AST_QUEUE:
case AST_STACK:
case AST_BRANCH:
                if (!runtime_execute_state_operation(
                        runtime,
                        child)) {

                    return 0;
                }
                break;

            /*
             * These are recognized but not executed
             * yet. Their runtime systems come later.
             */
            case AST_EXPRESSION:
            case AST_TRANSITION:
            case AST_CONTEXT:
            case AST_IF:
            case AST_ELSE_IF:
            case AST_ELSE:
            case AST_CONTRACT_CALL:
            case AST_RESULT:
            case AST_TERMINATE:
                break;

            default:
                break;
        }
    }

    return 1;
}


/*
 * Execute the EXECUTE statement.
 *
 * For now this is simply the final runtime marker.
 */
int runtime_execute_execute(
    Runtime *runtime,
    const ASTNode *execute_node)
{
    (void)runtime;
    (void)execute_node;

    printf(
        "EXECUTE\n"
    );

    return 1;
}


/*
 * Execute an entire Chaos program.
 */
int runtime_execute(
    Runtime *runtime,
    const ASTNode *program)
{
    if (runtime == NULL ||
        program == NULL) {

        return 0;
    }

    if (program->type != AST_PROGRAM) {
        return 0;
    }

    for (size_t i = 0;
         i < program->child_count;
         i++) {

        const ASTNode *node =
            program->children[i];

        switch (node->type) {

            case AST_REGISTER:
                if (!runtime_execute_register(
                        runtime,
                        node)) {

                    return 0;
                }
                break;

            case AST_LOGIC:
                if (!runtime_execute_logic(
                        runtime,
                        node)) {

                    return 0;
                }
                break;

            case AST_EXECUTE:
                if (!runtime_execute_execute(
                        runtime,
                        node)) {

                    return 0;
                }
                break;

            default:
                break;
        }
    }

    return 1;
}


/*
 * Print all current runtime states.
 */
void runtime_print_state(
    const Runtime *runtime)
{
    if (runtime == NULL) {
        return;
    }

    runtime_state_print(
        runtime->states
    );
}
#include "runtime.h"

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/*
 * Convert an AST data-structure name into
 * the corresponding runtime value type.
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
 */
static RuntimeValueType runtime_scalar_type(
    const char *value)
{
    if (value == NULL) {
        return RUNTIME_VALUE_STRING;
    }

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
 * Add the initial contents of a data structure.
 *
 * The parser currently stores the collection contents
 * as one comma-separated AST value.
 */
static int runtime_initialize_collection(
    RuntimeState *state,
    const char *value)
{
    if (state == NULL ||
        value == NULL ||
        value[0] == '\0') {

        return 1;
    }

    char *contents = malloc(
        strlen(value) + 1
    );

    if (contents == NULL) {
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

        size_t length = strlen(item);

        while (length > 0 &&
               (item[length - 1] == ' ' ||
                item[length - 1] == '\n' ||
                item[length - 1] == '\r')) {

            item[length - 1] = '\0';
            length--;
        }

        /*
         * Remove surrounding single quotes.
         */
        length = strlen(item);

        if (length >= 2 &&
            item[0] == '\'' &&
            item[length - 1] == '\'') {

            item[length - 1] = '\0';
            item++;
        }

        if (!runtime_state_push(
                state,
                item)) {

            free(contents);
            return 0;
        }

        item = strtok(NULL, ",");
    }

    free(contents);

    return 1;
}

/*
 * Execute a REGISTER node.
 *
 * Example:
 *
 * register ('everything'):
 *
 *     state: integer = 42,
 *     state: fruits, list = {'apple', 'banana'};
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
            type = runtime_data_type(data_type);
        }
        else {
            type = runtime_scalar_type(value);
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
         * Populate initial list/queue/stack/branch
         * contents.
         */
        if (data_type != NULL &&
            value != NULL) {

            if (!runtime_initialize_collection(
                    runtime_state,
                    value)) {

                fprintf(
                    stderr,
                    "Runtime error: could not initialize state '%s'\n",
                    name
                );

                free(runtime_state->name);

                runtime_value_free_contents(
                    &runtime_state->value
                );

                free(runtime_state);

                return 0;
            }
        }

        runtime_state_store_add(
            runtime->states,
            runtime_state
        );
    }

    return 1;
}

/*
 * Execute one PUSH or POP action.
 */
static int runtime_execute_action(
    RuntimeState *state,
    const ASTNode *action)
{
    if (state == NULL ||
        action == NULL) {

        return 0;
    }

    /*
     * PUSH
     */
    if (action->type == AST_PUSH) {

        if (action->child_count == 0) {
            fprintf(
                stderr,
                "Runtime error: push requires a value\n"
            );

            return 0;
        }

        const ASTNode *value =
            action->children[0];

        if (value->value == NULL) {
            fprintf(
                stderr,
                "Runtime error: push value is empty\n"
            );

            return 0;
        }

        if (!runtime_state_push(
                state,
                value->value)) {

            fprintf(
                stderr,
                "Runtime error: could not push into '%s'\n",
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
 * Execute a data-structure operation group.
 *
 * New syntax:
 *
 * list fruits
 *     (push 'apple')
 *     (push 'banana'),
 *
 * AST:
 *
 * DATA STRUCTURE OPERATION: fruits
 *     DATA_TYPE: list
 *     PUSH
 *         VALUE: apple
 *     PUSH
 *         VALUE: banana
 */
int runtime_execute_data_structure_operation(
    Runtime *runtime,
    const ASTNode *operation)
{
    if (runtime == NULL ||
        operation == NULL) {

        return 0;
    }

    if (operation->type !=
        AST_DATA_STRUCTURE_OPERATION) {

        return 0;
    }

    if (operation->value == NULL) {
        fprintf(
            stderr,
            "Runtime error: data structure has no name\n"
        );

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
            "Runtime error: unknown data structure '%s'\n",
            operation->value
        );

        return 0;
    }

    const char *type_name = NULL;

    for (size_t i = 0;
         i < operation->child_count;
         i++) {

        const ASTNode *child =
            operation->children[i];

        if (child->type == AST_DATA_TYPE) {
            type_name = child->value;
            break;
        }
    }

    if (type_name == NULL) {
        fprintf(
            stderr,
            "Runtime error: data structure operation for '%s' has no type\n",
            state->name
        );

        return 0;
    }

    RuntimeValueType expected_type =
        runtime_data_type(type_name);

    if (state->value.type != expected_type) {
        fprintf(
            stderr,
            "Runtime error: type mismatch for state '%s': operation expects %s, state is %s\n",
            state->name,
            type_name,
            runtime_value_type_name(
                state->value.type
            )
        );

        return 0;
    }

    /*
     * The AST contains the data type as one child,
     * followed by any number of PUSH/POP operations.
     */
    for (size_t i = 0;
         i < operation->child_count;
         i++) {

        const ASTNode *child =
            operation->children[i];

        if (child->type == AST_DATA_TYPE) {
            continue;
        }

        if (child->type != AST_PUSH &&
            child->type != AST_POP) {

            continue;
        }

        if (!runtime_execute_action(
                state,
                child)) {

            return 0;
        }
    }

    return 1;
}

/*
 * Execute a CONSTANT.
 *
 * Constants are reported in v1; expression evaluation is outside
 * the current runtime mutation surface.
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
 * Execute a LOGIC node.
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

            case AST_DATA_STRUCTURE_OPERATION:
                if (!runtime_execute_data_structure_operation(
                        runtime,
                        child)) {

                    return 0;
                }
                break;

            /*
             * These systems are recognized by the parser
             * but are not runtime semantics yet.
             */
            case AST_EXPRESSION:
            case AST_TRANSITION:
            case AST_CONTEXT:
            case AST_RULE:
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
 */
int runtime_execute_execute(
    Runtime *runtime,
    const ASTNode *execute_node)
{
    (void)runtime;
    (void)execute_node;

    printf("EXECUTE\n");

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

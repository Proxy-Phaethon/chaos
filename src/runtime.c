#include "runtime.h"

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

static RuntimeValueType runtime_data_type(const char *type)
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

static RuntimeValueType runtime_scalar_type(const char *value)
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

static char *copy_string(const char *source)
{
    if (source == NULL) {
        return NULL;
    }

    char *result = malloc(strlen(source) + 1);

    if (result == NULL) {
        return NULL;
    }

    strcpy(result, source);
    return result;
}

Runtime *runtime_create(void)
{
    Runtime *runtime = calloc(1, sizeof(Runtime));

    if (runtime == NULL) {
        return NULL;
    }

    runtime->states = runtime_state_store_create();

    if (runtime->states == NULL) {
        free(runtime);
        return NULL;
    }

    runtime->current_state = NULL;
    return runtime;
}

void runtime_free(Runtime *runtime)
{
    if (runtime == NULL) {
        return;
    }

    runtime_state_store_free(runtime->states);
    free(runtime->current_state);
    free(runtime);
}

static void trim_item(char *item)
{
    if (item == NULL) {
        return;
    }

    while (*item == ' ' || *item == '\t' || *item == '\n' || *item == '\r') {
        memmove(item, item + 1, strlen(item));
    }

    size_t length = strlen(item);

    while (length > 0 &&
           (item[length - 1] == ' ' ||
            item[length - 1] == '\t' ||
            item[length - 1] == '\n' ||
            item[length - 1] == '\r')) {
        item[--length] = '\0';
    }

    length = strlen(item);

    if (length >= 2 && item[0] == '\'' && item[length - 1] == '\'') {
        memmove(item, item + 1, length - 1);
        item[length - 2] = '\0';
    }
}

static int runtime_initialize_collection(
    RuntimeState *state,
    const char *value)
{
    if (state == NULL || value == NULL) {
        return 0;
    }

    if (state->value.type != RUNTIME_VALUE_LIST &&
        state->value.type != RUNTIME_VALUE_QUEUE &&
        state->value.type != RUNTIME_VALUE_STACK &&
        state->value.type != RUNTIME_VALUE_BRANCH) {
        return 1;
    }

    char *contents = copy_string(value);

    if (contents == NULL) {
        return 0;
    }

    char *item = strtok(contents, ",");

    while (item != NULL) {
        trim_item(item);

        if (item[0] == '\0') {
            free(contents);
            return 0;
        }

        if (!runtime_state_push(state, item)) {
            free(contents);
            return 0;
        }

        item = strtok(NULL, ",");
    }

    free(contents);
    return 1;
}

RuntimeResult runtime_execute_register(
    Runtime *runtime,
    const ASTNode *register_node)
{
    if (runtime == NULL || register_node == NULL ||
        register_node->type != AST_REGISTER) {
        return RUNTIME_ERROR;
    }

    for (size_t i = 0; i < register_node->child_count; i++) {
        const ASTNode *state = register_node->children[i];

        if (state == NULL || state->type != AST_STATE_DECLARATION) {
            continue;
        }

        const char *name = NULL;
        const char *value = NULL;
        const char *data_type = NULL;
        ASTType value_type = AST_STATE_VALUE;

        for (size_t j = 0; j < state->child_count; j++) {
            const ASTNode *child = state->children[j];

            if (child == NULL) {
                continue;
            }

            switch (child->type) {
                case AST_STATE_NAME:
                    name = child->value;
                    break;
                case AST_STATE_VALUE:
                case AST_EXPRESSION:
                    value = child->value;
                    value_type = child->type;
                    break;
                case AST_DATA_TYPE:
                    data_type = child->value;
                    break;
                default:
                    break;
            }
        }

        if (name == NULL || value == NULL) {
            fprintf(stderr, "Runtime error: incomplete state declaration\n");
            return RUNTIME_ERROR;
        }

        if (runtime_state_find(runtime->states, name) != NULL) {
            fprintf(stderr, "Runtime error: state '%s' already exists\n", name);
            return RUNTIME_ERROR;
        }

        RuntimeValueType type;

        if (data_type != NULL) {
            type = runtime_data_type(data_type);
        }
        else if (value_type == AST_EXPRESSION) {
            type = RUNTIME_VALUE_EXPRESSION;
        }
        else {
            type = runtime_scalar_type(value);
        }

        RuntimeState *runtime_state = runtime_state_create(name, type, value);

        if (runtime_state == NULL) {
            fprintf(stderr, "Runtime error: could not create state '%s'\n", name);
            return RUNTIME_ERROR;
        }

        if (data_type != NULL &&
            !runtime_initialize_collection(runtime_state, value)) {
            fprintf(stderr, "Runtime error: could not initialize state '%s'\n", name);
            runtime_state_free(runtime_state);
            return RUNTIME_ERROR;
        }

        if (!runtime_state_store_add(runtime->states, runtime_state)) {
            fprintf(stderr, "Runtime error: could not register state '%s'\n", name);
            runtime_state_free(runtime_state);
            return RUNTIME_ERROR;
        }
    }

    return RUNTIME_SUCCESS;
}

static RuntimeResult runtime_execute_action(
    RuntimeState *state,
    const ASTNode *action)
{
    if (state == NULL || action == NULL) {
        return RUNTIME_ERROR;
    }

    if (action->type == AST_PUSH) {
        if (action->child_count == 0 ||
            action->children[0] == NULL ||
            action->children[0]->value == NULL) {
            fprintf(stderr, "Runtime error: push requires a value\n");
            return RUNTIME_ERROR;
        }

        if (!runtime_state_push(state, action->children[0]->value)) {
            fprintf(stderr, "Runtime error: could not push into '%s'\n", state->name);
            return RUNTIME_ERROR;
        }

        return RUNTIME_SUCCESS;
    }

    if (action->type == AST_POP) {
        char *value = runtime_state_pop(state);

        if (value == NULL) {
            fprintf(stderr, "Runtime error: cannot pop empty state '%s'\n", state->name);
            return RUNTIME_ERROR;
        }

        printf("POP %s: %s\n", state->name, value);
        free(value);
        return RUNTIME_SUCCESS;
    }

    return RUNTIME_ERROR;
}

RuntimeResult runtime_execute_data_structure_operation(
    Runtime *runtime,
    const ASTNode *operation)
{
    if (runtime == NULL || operation == NULL ||
        operation->type != AST_DATA_STRUCTURE_OPERATION ||
        operation->value == NULL) {
        return RUNTIME_ERROR;
    }

    RuntimeState *state = runtime_state_find(runtime->states, operation->value);

    if (state == NULL) {
        fprintf(stderr, "Runtime error: unknown data structure '%s'\n", operation->value);
        return RUNTIME_ERROR;
    }

    const char *type_name = NULL;

    for (size_t i = 0; i < operation->child_count; i++) {
        const ASTNode *child = operation->children[i];

        if (child != NULL && child->type == AST_DATA_TYPE) {
            type_name = child->value;
            break;
        }
    }

    if (type_name == NULL) {
        fprintf(stderr, "Runtime error: data structure operation for '%s' has no type\n", state->name);
        return RUNTIME_ERROR;
    }

    RuntimeValueType expected = runtime_data_type(type_name);

    if (state->value.type != expected) {
        fprintf(
            stderr,
            "Runtime error: type mismatch for state '%s': operation expects %s, state is %s\n",
            state->name,
            type_name,
            runtime_value_type_name(state->value.type)
        );
        return RUNTIME_ERROR;
    }

    for (size_t i = 0; i < operation->child_count; i++) {
        const ASTNode *child = operation->children[i];

        if (child == NULL || child->type == AST_DATA_TYPE) {
            continue;
        }

        if (child->type == AST_PUSH || child->type == AST_POP) {
            RuntimeResult result = runtime_execute_action(state, child);

            if (result != RUNTIME_SUCCESS) {
                return result;
            }
        }
    }

    return RUNTIME_SUCCESS;
}

RuntimeResult runtime_execute_constant(
    Runtime *runtime,
    const ASTNode *constant)
{
    (void)runtime;

    if (constant == NULL || constant->type != AST_CONSTANT ||
        constant->child_count == 0 || constant->children[0] == NULL) {
        return RUNTIME_ERROR;
    }

    printf(
        "CONSTANT: %s\n",
        constant->children[0]->value != NULL
            ? constant->children[0]->value
            : ""
    );

    return RUNTIME_SUCCESS;
}

RuntimeResult runtime_execute_transition(
    Runtime *runtime,
    const ASTNode *transition)
{
    if (runtime == NULL || transition == NULL ||
        transition->type != AST_TRANSITION ||
        transition->value == NULL) {
        return RUNTIME_ERROR;
    }

    char *name = copy_string(transition->value);

    if (name == NULL) {
        return RUNTIME_ERROR;
    }

    free(runtime->current_state);
    runtime->current_state = name;

    printf("TRANSITION: %s\n", transition->value);
    return RUNTIME_SUCCESS;
}

RuntimeResult runtime_execute_context(
    Runtime *runtime,
    const ASTNode *context)
{
    (void)runtime;

    if (context == NULL || context->type != AST_CONTEXT) {
        return RUNTIME_ERROR;
    }

    return RUNTIME_SUCCESS;
}

RuntimeResult runtime_execute_rule(
    Runtime *runtime,
    const ASTNode *rule)
{
    (void)runtime;

    if (rule == NULL || rule->type != AST_RULE) {
        return RUNTIME_ERROR;
    }

    return RUNTIME_SUCCESS;
}

RuntimeResult runtime_execute_contract(
    Runtime *runtime,
    const ASTNode *contract)
{
    (void)runtime;

    if (contract == NULL || contract->type != AST_CONTRACT_CALL) {
        return RUNTIME_ERROR;
    }

    return RUNTIME_SUCCESS;
}

RuntimeResult runtime_execute_result(
    Runtime *runtime,
    const ASTNode *result)
{
    (void)runtime;

    if (result == NULL || result->type != AST_RESULT) {
        return RUNTIME_ERROR;
    }

    return RUNTIME_SUCCESS;
}

RuntimeResult runtime_execute_terminate(
    Runtime *runtime,
    const ASTNode *terminate)
{
    (void)runtime;

    if (terminate == NULL || terminate->type != AST_TERMINATE) {
        return RUNTIME_ERROR;
    }

    printf("TERMINATE\n");
    return RUNTIME_TERMINATE;
}

static RuntimeResult runtime_execute_node(
    Runtime *runtime,
    const ASTNode *node)
{
    if (runtime == NULL || node == NULL) {
        return RUNTIME_ERROR;
    }

    switch (node->type) {
        case AST_CONSTANT:
            return runtime_execute_constant(runtime, node);
        case AST_DATA_STRUCTURE_OPERATION:
            return runtime_execute_data_structure_operation(runtime, node);
        case AST_TRANSITION:
            return runtime_execute_transition(runtime, node);
        case AST_CONTEXT:
            return runtime_execute_context(runtime, node);
        case AST_RULE:
            return runtime_execute_rule(runtime, node);
        case AST_CONTRACT_CALL:
            return runtime_execute_contract(runtime, node);
        case AST_RESULT:
            return runtime_execute_result(runtime, node);
        case AST_TERMINATE:
            return runtime_execute_terminate(runtime, node);
        case AST_IF:
        case AST_ELSE_IF:
        case AST_ELSE:
        case AST_EXPRESSION:
            /* Conditions and expressions remain structural in v1. */
            return RUNTIME_SUCCESS;
        default:
            return RUNTIME_SUCCESS;
    }
}

RuntimeResult runtime_execute_logic(
    Runtime *runtime,
    const ASTNode *logic_node)
{
    if (runtime == NULL || logic_node == NULL ||
        logic_node->type != AST_LOGIC) {
        return RUNTIME_ERROR;
    }

    /* Child zero is the logic condition. It is structural in v1. */
    size_t start = logic_node->child_count > 0 ? 1 : 0;

    for (size_t i = start; i < logic_node->child_count; i++) {
        RuntimeResult result = runtime_execute_node(
            runtime,
            logic_node->children[i]
        );

        if (result != RUNTIME_SUCCESS) {
            return result;
        }
    }

    return RUNTIME_SUCCESS;
}

RuntimeResult runtime_execute_execute(
    Runtime *runtime,
    const ASTNode *execute_node)
{
    (void)runtime;

    if (execute_node == NULL || execute_node->type != AST_EXECUTE) {
        return RUNTIME_ERROR;
    }

    printf("EXECUTE\n");
    return RUNTIME_SUCCESS;
}

RuntimeResult runtime_execute(
    Runtime *runtime,
    const ASTNode *program)
{
    if (runtime == NULL || program == NULL ||
        program->type != AST_PROGRAM) {
        return RUNTIME_ERROR;
    }

    for (size_t i = 0; i < program->child_count; i++) {
        const ASTNode *node = program->children[i];

        if (node == NULL) {
            continue;
        }

        RuntimeResult result;

        switch (node->type) {
            case AST_REGISTER:
                result = runtime_execute_register(runtime, node);
                break;
            case AST_LOGIC:
                result = runtime_execute_logic(runtime, node);
                break;
            case AST_EXECUTE:
                result = runtime_execute_execute(runtime, node);
                break;
            case AST_TERMINATE:
                result = runtime_execute_terminate(runtime, node);
                break;
            default:
                result = RUNTIME_SUCCESS;
                break;
        }

        if (result != RUNTIME_SUCCESS) {
            return result;
        }
    }

    return RUNTIME_SUCCESS;
}

void runtime_print_state(const Runtime *runtime)
{
    if (runtime == NULL) {
        return;
    }

    runtime_state_print(runtime->states);
}

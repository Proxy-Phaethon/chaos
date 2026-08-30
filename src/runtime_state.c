#include "runtime_state.h"

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

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

static int is_collection_type(RuntimeValueType type)
{
    return type == RUNTIME_VALUE_LIST ||
           type == RUNTIME_VALUE_QUEUE ||
           type == RUNTIME_VALUE_STACK ||
           type == RUNTIME_VALUE_BRANCH;
}

static int ensure_capacity(RuntimeValue *value)
{
    if (value == NULL || !is_collection_type(value->type)) {
        return 0;
    }

    if (value->item_count < value->item_capacity) {
        return 1;
    }

    size_t new_capacity =
        value->item_capacity == 0 ? 4 : value->item_capacity * 2;

    char **new_items = realloc(
        value->items,
        new_capacity * sizeof(char *)
    );

    if (new_items == NULL) {
        return 0;
    }

    value->items = new_items;
    value->item_capacity = new_capacity;
    return 1;
}

RuntimeValue *runtime_value_create(
    RuntimeValueType type,
    const char *value)
{
    RuntimeValue *result = calloc(1, sizeof(RuntimeValue));

    if (result == NULL) {
        return NULL;
    }

    result->type = type;

    if (!is_collection_type(type) && value != NULL) {
        result->scalar = copy_string(value);

        if (result->scalar == NULL) {
            free(result);
            return NULL;
        }
    }

    return result;
}

void runtime_value_free_contents(RuntimeValue *value)
{
    if (value == NULL) {
        return;
    }

    free(value->scalar);
    value->scalar = NULL;

    if (value->items != NULL) {
        for (size_t i = 0; i < value->item_count; i++) {
            free(value->items[i]);
        }
        free(value->items);
    }

    value->items = NULL;
    value->item_count = 0;
    value->item_capacity = 0;
    value->branch_root = NULL;
    value->branch_count = 0;
}

void runtime_value_free(RuntimeValue *value)
{
    if (value == NULL) {
        return;
    }

    runtime_value_free_contents(value);
    free(value);
}

const char *runtime_value_type_name(RuntimeValueType type)
{
    switch (type) {
        case RUNTIME_VALUE_NUMBER:
            return "number";
        case RUNTIME_VALUE_STRING:
            return "string";
        case RUNTIME_VALUE_EXPRESSION:
            return "expression";
        case RUNTIME_VALUE_LIST:
            return "list";
        case RUNTIME_VALUE_QUEUE:
            return "queue";
        case RUNTIME_VALUE_STACK:
            return "stack";
        case RUNTIME_VALUE_BRANCH:
            return "branch";
        default:
            return "unknown";
    }
}

RuntimeState *runtime_state_create(
    const char *name,
    RuntimeValueType type,
    const char *value)
{
    if (name == NULL) {
        return NULL;
    }

    RuntimeState *state = calloc(1, sizeof(RuntimeState));

    if (state == NULL) {
        return NULL;
    }

    state->name = copy_string(name);

    if (state->name == NULL) {
        free(state);
        return NULL;
    }

    RuntimeValue *runtime_value = runtime_value_create(type, value);

    if (runtime_value == NULL) {
        free(state->name);
        free(state);
        return NULL;
    }

    state->value = *runtime_value;
    free(runtime_value);
    return state;
}

void runtime_state_free(RuntimeState *state)
{
    if (state == NULL) {
        return;
    }

    free(state->name);
    runtime_value_free_contents(&state->value);
    free(state);
}

RuntimeStateStore *runtime_state_store_create(void)
{
    return calloc(1, sizeof(RuntimeStateStore));
}

int runtime_state_store_add(
    RuntimeStateStore *store,
    RuntimeState *state)
{
    if (store == NULL || state == NULL || state->name == NULL) {
        return 0;
    }

    if (runtime_state_find(store, state->name) != NULL) {
        return 0;
    }

    state->next = store->head;
    store->head = state;
    return 1;
}

RuntimeState *runtime_state_find(
    RuntimeStateStore *store,
    const char *name)
{
    if (store == NULL || name == NULL) {
        return NULL;
    }

    RuntimeState *current = store->head;

    while (current != NULL) {
        if (strcmp(current->name, name) == 0) {
            return current;
        }
        current = current->next;
    }

    return NULL;
}

void runtime_state_store_free(RuntimeStateStore *store)
{
    if (store == NULL) {
        return;
    }

    RuntimeState *current = store->head;

    while (current != NULL) {
        RuntimeState *next = current->next;
        runtime_state_free(current);
        current = next;
    }

    free(store);
}

int runtime_state_set_value(
    RuntimeState *state,
    const char *value)
{
    if (state == NULL || value == NULL) {
        return 0;
    }

    if (is_collection_type(state->value.type)) {
        return 0;
    }

    char *new_value = copy_string(value);

    if (new_value == NULL) {
        return 0;
    }

    free(state->value.scalar);
    state->value.scalar = new_value;
    return 1;
}

const char *runtime_state_get_value(const RuntimeState *state)
{
    if (state == NULL) {
        return NULL;
    }

    return state->value.scalar;
}

int runtime_state_push(
    RuntimeState *state,
    const char *value)
{
    if (state == NULL || value == NULL || !is_collection_type(state->value.type)) {
        return 0;
    }

    if (!ensure_capacity(&state->value)) {
        return 0;
    }

    char *item = copy_string(value);

    if (item == NULL) {
        return 0;
    }

    state->value.items[state->value.item_count++] = item;
    state->value.branch_count = state->value.item_count;
    return 1;
}

char *runtime_state_pop(RuntimeState *state)
{
    if (state == NULL || !is_collection_type(state->value.type)) {
        return NULL;
    }

    RuntimeValue *value = &state->value;

    if (value->item_count == 0) {
        return NULL;
    }

    size_t index =
        value->type == RUNTIME_VALUE_STACK
        ? value->item_count - 1
        : 0;

    char *result = value->items[index];

    if (index == 0) {
        for (size_t i = 1; i < value->item_count; i++) {
            value->items[i - 1] = value->items[i];
        }
    }

    value->item_count--;
    value->items[value->item_count] = NULL;
    value->branch_count = value->item_count;
    return result;
}

size_t runtime_state_count(const RuntimeState *state)
{
    if (state == NULL) {
        return 0;
    }

    return state->value.item_count;
}

int runtime_branch_insert(
    RuntimeState *state,
    const char *value)
{
    if (state == NULL || state->value.type != RUNTIME_VALUE_BRANCH) {
        return 0;
    }

    return runtime_state_push(state, value);
}

int runtime_branch_contains(
    const RuntimeState *state,
    const char *value)
{
    if (state == NULL || value == NULL ||
        state->value.type != RUNTIME_VALUE_BRANCH) {
        return 0;
    }

    for (size_t i = 0; i < state->value.item_count; i++) {
        if (strcmp(state->value.items[i], value) == 0) {
            return 1;
        }
    }

    return 0;
}

void runtime_branch_print(const RuntimeState *state)
{
    if (state == NULL || state->value.type != RUNTIME_VALUE_BRANCH) {
        return;
    }

    printf("{");

    for (size_t i = 0; i < state->value.item_count; i++) {
        if (i > 0) {
            printf(", ");
        }
        printf("%s", state->value.items[i]);
    }

    printf("}");
}

void runtime_state_print(const RuntimeStateStore *store)
{
    if (store == NULL) {
        return;
    }

    const RuntimeState *current = store->head;

    while (current != NULL) {
        printf(
            "%s [%s]",
            current->name,
            runtime_value_type_name(current->value.type)
        );

        if (current->value.scalar != NULL) {
            printf(" = %s", current->value.scalar);
        }
        else if (current->value.type == RUNTIME_VALUE_BRANCH) {
            printf(" = ");
            runtime_branch_print(current);
        }
        else if (current->value.item_count > 0) {
            printf(" = {");

            for (size_t i = 0; i < current->value.item_count; i++) {
                if (i > 0) {
                    printf(", ");
                }
                printf("%s", current->value.items[i]);
            }

            printf("}");
        }

        printf("\n");
        current = current->next;
    }
}

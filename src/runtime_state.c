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

static int ensure_capacity(RuntimeValue *value)
{
    if (value->item_count < value->item_capacity) {
        return 1;
    }

    size_t new_capacity =
        value->item_capacity == 0
        ? 4
        : value->item_capacity * 2;

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
    RuntimeValue *result = calloc(
        1,
        sizeof(RuntimeValue)
    );

    if (result == NULL) {
        return NULL;
    }

    result->type = type;

    if (type == RUNTIME_VALUE_LIST ||
        type == RUNTIME_VALUE_QUEUE ||
        type == RUNTIME_VALUE_STACK ||
        type == RUNTIME_VALUE_BRANCH) {

        /*
         * Collection contents are populated separately
         * by the runtime.
         */
        (void)value;

        return result;
    }

    if (value != NULL) {
        result->scalar = copy_string(value);

        if (result->scalar == NULL) {
            free(result);
            return NULL;
        }
    }

    return result;
}

void runtime_value_free(RuntimeValue *value)
{
    if (value == NULL) {
        return;
    }

    free(value->scalar);

    for (size_t i = 0;
         i < value->item_count;
         i++) {

        free(value->items[i]);
    }

    free(value->items);
    free(value);
}

RuntimeState *runtime_state_create(
    const char *name,
    RuntimeValueType type,
    const char *value)
{
    RuntimeState *state = calloc(
        1,
        sizeof(RuntimeState)
    );

    if (state == NULL) {
        return NULL;
    }

    state->name = copy_string(name);

    if (state->name == NULL) {
        free(state);
        return NULL;
    }

    RuntimeValue *runtime_value =
        runtime_value_create(type, value);

    if (runtime_value == NULL) {
        free(state->name);
        free(state);
        return NULL;
    }

    /*
     * Move the RuntimeValue contents into the state.
     */
    state->value = *runtime_value;

    /*
     * The RuntimeValue structure itself is no
     * longer needed because its contents now belong
     * to the state.
     */
    free(runtime_value);

    state->next = NULL;

    return state;
}

RuntimeStateStore *runtime_state_store_create(void)
{
    return calloc(
        1,
        sizeof(RuntimeStateStore)
    );
}

void runtime_state_store_add(
    RuntimeStateStore *store,
    RuntimeState *state)
{
    if (store == NULL || state == NULL) {
        return;
    }

    state->next = store->head;
    store->head = state;
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

int runtime_state_push(
    RuntimeState *state,
    const char *value)
{
    if (state == NULL || value == NULL) {
        return 0;
    }

    RuntimeValue *runtime_value = &state->value;

    if (runtime_value->type != RUNTIME_VALUE_LIST &&
        runtime_value->type != RUNTIME_VALUE_QUEUE &&
        runtime_value->type != RUNTIME_VALUE_STACK &&
        runtime_value->type != RUNTIME_VALUE_BRANCH) {

        return 0;
    }

    if (!ensure_capacity(runtime_value)) {
        return 0;
    }

    char *item = copy_string(value);

    if (item == NULL) {
        return 0;
    }

    runtime_value->items[
        runtime_value->item_count
    ] = item;

    runtime_value->item_count++;

    return 1;
}

char *runtime_state_pop(
    RuntimeState *state)
{
    if (state == NULL) {
        return NULL;
    }

    RuntimeValue *value = &state->value;

    if (value->item_count == 0) {
        return NULL;
    }

    if (value->type != RUNTIME_VALUE_LIST &&
        value->type != RUNTIME_VALUE_QUEUE &&
        value->type != RUNTIME_VALUE_STACK &&
        value->type != RUNTIME_VALUE_BRANCH) {

        return NULL;
    }

    size_t index;

    /*
     * Stack = last in, first out.
     */
    if (value->type == RUNTIME_VALUE_STACK) {
        index = value->item_count - 1;
    }
    /*
     * Queue = first in, first out.
     *
     * Lists and branches currently use the
     * first item as their removal position.
     */
    else {
        index = 0;
    }

    char *result = value->items[index];

    /*
     * Removing the first item requires shifting
     * everything else toward the beginning.
     */
    if (index == 0) {
        for (size_t i = 1;
             i < value->item_count;
             i++) {

            value->items[i - 1] = value->items[i];
        }
    }

    value->item_count--;

    value->items[value->item_count] = NULL;

    return result;
}

void runtime_state_store_free(
    RuntimeStateStore *store)
{
    if (store == NULL) {
        return;
    }

    RuntimeState *current = store->head;

    while (current != NULL) {
        RuntimeState *next = current->next;

        free(current->name);

        free(current->value.scalar);

        for (size_t i = 0;
             i < current->value.item_count;
             i++) {

            free(current->value.items[i]);
        }

        free(current->value.items);
        free(current);

        current = next;
    }

    free(store);
}

static const char *runtime_value_type_name(
    RuntimeValueType type)
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

void runtime_state_print(
    const RuntimeStateStore *store)
{
    if (store == NULL) {
        return;
    }

    const RuntimeState *current = store->head;

    while (current != NULL) {
        printf(
            "%s [%s]",
            current->name,
            runtime_value_type_name(
                current->value.type
            )
        );

        if (current->value.scalar != NULL) {
            printf(
                " = %s",
                current->value.scalar
            );
        }

        if (current->value.item_count > 0) {
            printf(" = {");

            for (size_t i = 0;
                 i < current->value.item_count;
                 i++) {

                if (i > 0) {
                    printf(", ");
                }

                printf(
                    "%s",
                    current->value.items[i]
                );
            }

            printf("}");
        }

        printf("\n");

        current = current->next;
    }
}

void runtime_state_store_free(RuntimeStateStore *store)
{
    if (store == NULL) {
        return;
    }

    RuntimeState *current = store->head;

    while (current != NULL) {
        RuntimeState *next = current->next;

        free(current->name);
        runtime_value_free_contents(&current->value);
        free(current);

        current = next;
    }

    free(store);
}
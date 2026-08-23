#include "runtime_state.h"

#include <stdio.h>
#include <stdlib.h>
#include <string.h>


/*
 * Copy a string into newly allocated memory.
 */
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


/*
 * Make sure a collection has room for another item.
 */
static int ensure_capacity(RuntimeValue *value)
{
    if (value == NULL) {
        return 0;
    }

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


/*
 * Create a runtime value.
 *
 * Scalar values use:
 *
 *     scalar
 *
 * Data structures use:
 *
 *     items
 *     item_count
 *     item_capacity
 *
 * Initial collection contents are populated later
 * by the runtime when the AST is executed.
 */
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

        result->items = NULL;
        result->item_count = 0;
        result->item_capacity = 0;

        /*
         * Collection contents will be populated
         * by the runtime.
         */
        (void)value;

        return result;
    }

    /*
     * Scalar value.
     */
    if (value != NULL) {
        result->scalar = copy_string(value);

        if (result->scalar == NULL) {
            free(result);
            return NULL;
        }
    }

    return result;
}


/*
 * Free the contents owned by a RuntimeValue.
 *
 * This does NOT free the RuntimeValue itself.
 */
void runtime_value_free_contents(RuntimeValue *value)
{
    if (value == NULL) {
        return;
    }

    free(value->scalar);
    value->scalar = NULL;

    for (size_t i = 0; i < value->item_count; i++) {
        free(value->items[i]);
    }

    free(value->items);

    value->items = NULL;
    value->item_count = 0;
    value->item_capacity = 0;
}


/*
 * Free an entire RuntimeValue.
 */
void runtime_value_free(RuntimeValue *value)
{
    if (value == NULL) {
        return;
    }

    runtime_value_free_contents(value);

    free(value);
}


/*
 * Create a runtime state.
 */
RuntimeState *runtime_state_create(
    const char *name,
    RuntimeValueType type,
    const char *value)
{
    if (name == NULL) {
        return NULL;
    }

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
     * Move the RuntimeValue contents into
     * the state rather than allocating another
     * RuntimeValue.
     */
    state->value = *runtime_value;

    free(runtime_value);

    state->next = NULL;

    return state;
}


/*
 * Create an empty runtime state store.
 */
RuntimeStateStore *runtime_state_store_create(void)
{
    return calloc(
        1,
        sizeof(RuntimeStateStore)
    );
}


/*
 * Add a state to the store.
 */
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


/*
 * Find a state by name.
 */
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


/*
 * Free the entire state store.
 */
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

        runtime_value_free_contents(
            &current->value
        );

        free(current);

        current = next;
    }

    free(store);
}


/*
 * Push a value onto a collection.
 *
 * For now, all collection types use the same
 * underlying storage mechanism.
 *
 * Their actual semantics will be implemented
 * by the runtime:
 *
 *     list
 *     queue
 *     stack
 *     branch
 */
int runtime_state_push(
    RuntimeState *state,
    const char *value)
{
    if (state == NULL || value == NULL) {
        return 0;
    }

    RuntimeValue *runtime_value =
        &state->value;

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


/*
 * Pop a value from a collection.
 *
 * Stack:
 *     removes newest item.
 *
 * Queue:
 *     removes oldest item.
 *
 * List:
 *     currently removes oldest item.
 *
 * Branch:
 *     currently removes oldest item.
 *
 * List and branch semantics will be refined
 * when their runtime implementations are built.
 */
char *runtime_state_pop(
    RuntimeState *state)
{
    if (state == NULL) {
        return NULL;
    }

    RuntimeValue *value =
        &state->value;

    if (value->type != RUNTIME_VALUE_LIST &&
        value->type != RUNTIME_VALUE_QUEUE &&
        value->type != RUNTIME_VALUE_STACK &&
        value->type != RUNTIME_VALUE_BRANCH) {

        return NULL;
    }

    if (value->item_count == 0) {
        return NULL;
    }

    size_t index;

    if (value->type == RUNTIME_VALUE_STACK) {
        /*
         * Stack = LIFO.
         */
        index = value->item_count - 1;
    }
    else {
        /*
         * Queue/list/branch currently remove
         * the oldest stored item.
         */
        index = 0;
    }

    char *result = value->items[index];

    /*
     * If removing the first item, shift the
     * remaining items one position to the left.
     */
    if (index == 0) {
        for (size_t i = 1;
             i < value->item_count;
             i++) {

            value->items[i - 1] =
                value->items[i];
        }
    }

    value->item_count--;

    value->items[
        value->item_count
    ] = NULL;

    return result;
}


/*
 * Return a human-readable runtime value type.
 */
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


/*
 * Print the current runtime state store.
 */
void runtime_state_print(
    const RuntimeStateStore *store)
{
    if (store == NULL) {
        return;
    }

    const RuntimeState *current =
        store->head;

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
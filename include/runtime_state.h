#ifndef CHAOS_RUNTIME_STATE_H
#define CHAOS_RUNTIME_STATE_H

#include <stddef.h>

typedef enum {
    RUNTIME_VALUE_NUMBER,
    RUNTIME_VALUE_STRING,
    RUNTIME_VALUE_EXPRESSION,

    RUNTIME_VALUE_LIST,
    RUNTIME_VALUE_QUEUE,
    RUNTIME_VALUE_STACK,
    RUNTIME_VALUE_BRANCH
} RuntimeValueType;

typedef struct RuntimeValue {
    RuntimeValueType type;

    char *scalar;

    char **items;
    size_t item_count;
    size_t item_capacity;
} RuntimeValue;

typedef struct RuntimeState {
    char *name;
    RuntimeValue value;

    struct RuntimeState *next;
} RuntimeState;

typedef struct {
    RuntimeState *head;
} RuntimeStateStore;

RuntimeStateStore *runtime_state_store_create(void);

void runtime_state_store_free(
    RuntimeStateStore *store
);

void runtime_state_store_add(
    RuntimeStateStore *store,
    RuntimeState *state
);

RuntimeState *runtime_state_find(
    RuntimeStateStore *store,
    const char *name
);

RuntimeValue *runtime_value_create(
    RuntimeValueType type,
    const char *value
);

void runtime_value_free(
    RuntimeValue *value
);

void runtime_value_free_contents(
    RuntimeValue *value
);

const char *runtime_value_type_name(
    RuntimeValueType type
);

RuntimeState *runtime_state_create(
    const char *name,
    RuntimeValueType type,
    const char *value
);

int runtime_state_push(
    RuntimeState *state,
    const char *value
);

char *runtime_state_pop(
    RuntimeState *state
);

void runtime_state_print(
    const RuntimeStateStore *store
);

#endif
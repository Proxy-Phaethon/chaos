#ifndef CHAOS_ACTION_H
#define CHAOS_ACTION_H

#include <stddef.h>

typedef struct {
    char *name;
} ChaosContract;

typedef struct {
    char *name;

    ChaosContract *contracts;
    size_t contract_count;
    size_t contract_capacity;
} ChaosAction;

typedef struct {
    ChaosAction *items;
    size_t count;
    size_t capacity;
} ChaosActionRegistry;

void chaos_actions_init(
    ChaosActionRegistry *registry
);

int chaos_action_register(
    ChaosActionRegistry *registry,
    const char *name
);

ChaosAction *chaos_action_find(
    ChaosActionRegistry *registry,
    const char *name
);

void chaos_actions_free(
    ChaosActionRegistry *registry
);

#endif
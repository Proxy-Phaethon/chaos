#include "action.h"

#include <stdlib.h>
#include <string.h>

static char *chaos_strdup(const char *source)
{
    if (source == NULL) {
        return NULL;
    }

    size_t length = strlen(source) + 1;
    char *copy = malloc(length);

    if (copy == NULL) {
        return NULL;
    }

    memcpy(copy, source, length);
    return copy;
}

void chaos_actions_init(
    ChaosActionRegistry *registry
)
{
    if (registry == NULL) {
        return;
    }

    registry->items = NULL;
    registry->count = 0;
    registry->capacity = 0;
}

int chaos_action_register(
    ChaosActionRegistry *registry,
    const char *name
)
{
    if (
        registry == NULL ||
        name == NULL ||
        name[0] == '\0'
    ) {
        return 0;
    }

    if (chaos_action_find(registry, name) != NULL) {
        return 1;
    }

    if (registry->count >= registry->capacity) {
        size_t new_capacity =
            registry->capacity == 0
                ? 8
                : registry->capacity * 2;

        ChaosAction *new_items =
            realloc(
                registry->items,
                new_capacity * sizeof(ChaosAction)
            );

        if (new_items == NULL) {
            return 0;
        }

        registry->items = new_items;
        registry->capacity = new_capacity;
    }

    ChaosAction *action =
        &registry->items[registry->count];

    action->name = chaos_strdup(name);
    action->contracts = NULL;
    action->contract_count = 0;
    action->contract_capacity = 0;

    if (action->name == NULL) {
        return 0;
    }

    registry->count++;

    return 1;
}

ChaosAction *chaos_action_find(
    ChaosActionRegistry *registry,
    const char *name
)
{
    if (registry == NULL || name == NULL) {
        return NULL;
    }

    for (size_t i = 0; i < registry->count; i++) {
        if (strcmp(registry->items[i].name, name) == 0) {
            return &registry->items[i];
        }
    }

    return NULL;
}

void chaos_actions_free(
    ChaosActionRegistry *registry
)
{
    if (registry == NULL) {
        return;
    }

    for (size_t i = 0; i < registry->count; i++) {
        ChaosAction *action = &registry->items[i];

        free(action->name);

        for (size_t j = 0; j < action->contract_count; j++) {
            free(action->contracts[j].name);
        }

        free(action->contracts);
    }

    free(registry->items);

    registry->items = NULL;
    registry->count = 0;
    registry->capacity = 0;
}
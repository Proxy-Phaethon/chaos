#include "condition.h"

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

void chaos_conditions_init(ChaosConditionRegistry *registry)
{
    if (registry == NULL) {
        return;
    }

    registry->items = NULL;
    registry->count = 0;
    registry->capacity = 0;
}

int chaos_condition_register(
    ChaosConditionRegistry *registry,
    const char *name
)
{
    if (registry == NULL || name == NULL || name[0] == '\0') {
        return 0;
    }

    if (chaos_condition_exists(registry, name)) {
        return 1;
    }

    if (registry->count >= registry->capacity) {
        size_t new_capacity =
            registry->capacity == 0
                ? 8
                : registry->capacity * 2;

        ChaosCondition *new_items =
            realloc(
                registry->items,
                new_capacity * sizeof(ChaosCondition)
            );

        if (new_items == NULL) {
            return 0;
        }

        registry->items = new_items;
        registry->capacity = new_capacity;
    }

    registry->items[registry->count].name = chaos_strdup(name);

    if (registry->items[registry->count].name == NULL) {
        return 0;
    }

    registry->count++;
    return 1;
}

int chaos_condition_exists(
    const ChaosConditionRegistry *registry,
    const char *name
)
{
    if (registry == NULL || name == NULL) {
        return 0;
    }

    for (size_t i = 0; i < registry->count; i++) {
        if (strcmp(registry->items[i].name, name) == 0) {
            return 1;
        }
    }

    return 0;
}

void chaos_conditions_free(
    ChaosConditionRegistry *registry
)
{
    if (registry == NULL) {
        return;
    }

    for (size_t i = 0; i < registry->count; i++) {
        free(registry->items[i].name);
    }

    free(registry->items);

    registry->items = NULL;
    registry->count = 0;
    registry->capacity = 0;
}
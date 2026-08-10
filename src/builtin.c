#include "builtin.h"

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

void chaos_builtins_init(
    ChaosBuiltinRegistry *registry
)
{
    if (registry == NULL) {
        return;
    }

    registry->items = NULL;
    registry->count = 0;
    registry->capacity = 0;
}

int chaos_builtin_register(
    ChaosBuiltinRegistry *registry,
    const char *name,
    ChaosBuiltinFunction function
)
{
    if (
        registry == NULL ||
        name == NULL ||
        function == NULL
    ) {
        return 0;
    }

    if (chaos_builtin_find(registry, name) != NULL) {
        return 1;
    }

    if (registry->count >= registry->capacity) {
        size_t new_capacity =
            registry->capacity == 0
                ? 8
                : registry->capacity * 2;

        ChaosBuiltin *new_items =
            realloc(
                registry->items,
                new_capacity * sizeof(ChaosBuiltin)
            );

        if (new_items == NULL) {
            return 0;
        }

        registry->items = new_items;
        registry->capacity = new_capacity;
    }

    registry->items[registry->count].name =
        chaos_strdup(name);

    if (registry->items[registry->count].name == NULL) {
        return 0;
    }

    registry->items[registry->count].function = function;
    registry->count++;

    return 1;
}

ChaosBuiltinFunction chaos_builtin_find(
    const ChaosBuiltinRegistry *registry,
    const char *name
)
{
    if (registry == NULL || name == NULL) {
        return NULL;
    }

    for (size_t i = 0; i < registry->count; i++) {
        if (strcmp(registry->items[i].name, name) == 0) {
            return registry->items[i].function;
        }
    }

    return NULL;
}

void chaos_builtins_free(
    ChaosBuiltinRegistry *registry
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
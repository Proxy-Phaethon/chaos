#ifndef CHAOS_CONDITION_H
#define CHAOS_CONDITION_H

#include <stddef.h>

typedef struct {
    char *name;
} ChaosCondition;

typedef struct {
    ChaosCondition *items;
    size_t count;
    size_t capacity;
} ChaosConditionRegistry;

/*
 * Initialise the condition registry.
 */
void chaos_conditions_init(ChaosConditionRegistry *registry);

/*
 * Register a condition.
 *
 * Conditions are data, not hardcoded branches.
 */
int chaos_condition_register(
    ChaosConditionRegistry *registry,
    const char *name
);

/*
 * Determine whether a named condition exists.
 */
int chaos_condition_exists(
    const ChaosConditionRegistry *registry,
    const char *name
);

/*
 * Free the registry.
 */
void chaos_conditions_free(
    ChaosConditionRegistry *registry
);

#endif
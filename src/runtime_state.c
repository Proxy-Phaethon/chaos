#include "runtime_state.h"

#include <stdio.h>
#include <stdlib.h>
#include <string.h>


static char *copy_string(const char *source)
{
    if (source == NULL) {
        return NULL;
    }

    char *result = malloc(
        strlen(source) + 1
    );

    if (result == NULL) {
        return NULL;
    }

    strcpy(result, source);

    return result;
}

static int is_collection_type(
    RuntimeValueType type)
{
    return type == RUNTIME_VALUE_LIST ||
           type == RUNTIME_VALUE_QUEUE ||
           type == RUNTIME_VALUE_STACK;
}


static int ensure_capacity(
    RuntimeValue *value)
{
    if (value == NULL) {
        return 0;
    }

    if (!is_collection_type(value->type)) {
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

static RuntimeBranchNode *branch_node_create(
    const char *value)
{
    if (value == NULL) {
        return NULL;
    }

    RuntimeBranchNode *node =
        calloc(
            1,
            sizeof(RuntimeBranchNode)
        );

    if (node == NULL) {
        return NULL;
    }

    node->value = copy_string(value);

    if (node->value == NULL) {
        free(node);
        return NULL;
    }

    node->left = NULL;
    node->right = NULL;

    return node;
}


static void branch_node_free(
    RuntimeBranchNode *node)
{
    if (node == NULL) {
        return;
    }

    branch_node_free(node->left);
    branch_node_free(node->right);

    free(node->value);
    free(node);
}


static RuntimeBranchNode *branch_insert_node(
    RuntimeBranchNode *root,
    const char *value,
    int *inserted)
{
    if (root == NULL) {
        RuntimeBranchNode *node =
            branch_node_create(value);

        if (node == NULL) {
            *inserted = 0;
            return NULL;
        }

        *inserted = 1;
        return node;
    }

    int comparison =
        strcmp(value, root->value);

    if (comparison < 0) {

        RuntimeBranchNode *child =
            branch_insert_node(
                root->left,
                value,
                inserted
            );

        if (*inserted) {
            root->left = child;
        }
    }
    else if (comparison > 0) {

        RuntimeBranchNode *child =
            branch_insert_node(
                root->right,
                value,
                inserted
            );

        if (*inserted) {
            root->right = child;
        }
    }
    else {

        *inserted = 0;
    }

    return root;
}


static int branch_contains_node(
    const RuntimeBranchNode *root,
    const char *value)
{
    if (root == NULL || value == NULL) {
        return 0;
    }

    int comparison =
        strcmp(value, root->value);

    if (comparison == 0) {
        return 1;
    }

    if (comparison < 0) {
        return branch_contains_node(
            root->left,
            value
        );
    }

    return branch_contains_node(
        root->right,
        value
    );
}


static void branch_print_inorder(
    const RuntimeBranchNode *node,
    int *first)
{
    if (node == NULL) {
        return;
    }

    branch_print_inorder(
        node->left,
        first
    );

    if (!*first) {
        printf(", ");
    }

    printf("%s", node->value);

    *first = 0;

    branch_print_inorder(
        node->right,
        first
    );
}

RuntimeValue *runtime_value_create(
    RuntimeValueType type,
    const char *value)
{
    RuntimeValue *result =
        calloc(
            1,
            sizeof(RuntimeValue)
        );

    if (result == NULL) {
        return NULL;
    }

    result->type = type;

    result->scalar = NULL;

    result->items = NULL;
    result->item_count = 0;
    result->item_capacity = 0;

    result->branch_root = NULL;
    result->branch_count = 0;

    if (type == RUNTIME_VALUE_LIST ||
        type == RUNTIME_VALUE_QUEUE ||
        type == RUNTIME_VALUE_STACK) {

        (void)value;

        return result;
    }

    if (type == RUNTIME_VALUE_BRANCH) {

        (void)value;

        return result;
    }

    if (value != NULL) {
        result->scalar =
            copy_string(value);

        if (result->scalar == NULL) {
            free(result);
            return NULL;
        }
    }

    return result;
}


void runtime_value_free_contents(
    RuntimeValue *value)
{
    if (value == NULL) {
        return;
    }

    free(value->scalar);
    value->scalar = NULL;

    if (value->items != NULL) {

        for (size_t i = 0;
             i < value->item_count;
             i++) {

            free(value->items[i]);
        }

        free(value->items);
    }

    value->items = NULL;
    value->item_count = 0;
    value->item_capacity = 0;

    branch_node_free(
        value->branch_root
    );

    value->branch_root = NULL;
    value->branch_count = 0;
}


void runtime_value_free(
    RuntimeValue *value)
{
    if (value == NULL) {
        return;
    }

    runtime_value_free_contents(value);

    free(value);
}


const char *runtime_value_type_name(
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

RuntimeState *runtime_state_create(
    const char *name,
    RuntimeValueType type,
    const char *value)
{
    if (name == NULL) {
        return NULL;
    }

    RuntimeState *state =
        calloc(
            1,
            sizeof(RuntimeState)
        );

    if (state == NULL) {
        return NULL;
    }

    state->name =
        copy_string(name);

    if (state->name == NULL) {
        free(state);
        return NULL;
    }

    RuntimeValue *runtime_value =
        runtime_value_create(
            type,
            value
        );

    if (runtime_value == NULL) {
        free(state->name);
        free(state);
        return NULL;
    }

    state->value = *runtime_value;

    free(runtime_value);

    state->next = NULL;

    return state;
}


void runtime_state_free(
    RuntimeState *state)
{
    if (state == NULL) {
        return;
    }

    free(state->name);

    runtime_value_free_contents(
        &state->value
    );

    free(state);
}

RuntimeStateStore *
runtime_state_store_create(void)
{
    return calloc(
        1,
        sizeof(RuntimeStateStore)
    );
}


int runtime_state_store_add(
    RuntimeStateStore *store,
    RuntimeState *state)
{
    if (store == NULL ||
        state == NULL ||
        state->name == NULL) {

        return 0;
    }

    if (runtime_state_find(
            store,
            state->name) != NULL) {

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
    if (store == NULL ||
        name == NULL) {

        return NULL;
    }

    RuntimeState *current =
        store->head;

    while (current != NULL) {

        if (strcmp(
                current->name,
                name
            ) == 0) {

            return current;
        }

        current = current->next;
    }

    return NULL;
}


void runtime_state_store_free(
    RuntimeStateStore *store)
{
    if (store == NULL) {
        return;
    }

    RuntimeState *current =
        store->head;

    while (current != NULL) {

        RuntimeState *next =
            current->next;

        runtime_state_free(current);

        current = next;
    }

    free(store);
}

int runtime_state_set_value(
    RuntimeState *state,
    const char *value)
{
    if (state == NULL ||
        value == NULL) {

        return 0;
    }

    if (state->value.type == RUNTIME_VALUE_LIST ||
        state->value.type == RUNTIME_VALUE_QUEUE ||
        state->value.type == RUNTIME_VALUE_STACK ||
        state->value.type == RUNTIME_VALUE_BRANCH) {

        return 0;
    }

    char *new_value =
        copy_string(value);

    if (new_value == NULL) {
        return 0;
    }

    free(state->value.scalar);

    state->value.scalar = new_value;

    return 1;
}


const char *runtime_state_get_value(
    const RuntimeState *state)
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
    if (state == NULL ||
        value == NULL) {

        return 0;
    }

    RuntimeValue *runtime_value =
        &state->value;

    if (!is_collection_type(
            runtime_value->type)) {

        return 0;
    }

    if (!ensure_capacity(
            runtime_value)) {

        return 0;
    }

    char *item =
        copy_string(value);

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

    RuntimeValue *value =
        &state->value;

    if (!is_collection_type(
            value->type)) {

        return NULL;
    }

    if (value->item_count == 0) {
        return NULL;
    }

    size_t index;

    if (value->type == RUNTIME_VALUE_STACK) {
        index = value->item_count - 1;
    }
    else {
        index = 0;
    }

    char *result =
        value->items[index];

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


size_t runtime_state_count(
    const RuntimeState *state)
{
    if (state == NULL) {
        return 0;
    }

    if (state->value.type ==
        RUNTIME_VALUE_BRANCH) {

        return state->value.branch_count;
    }

    return state->value.item_count;
}

int runtime_branch_insert(
    RuntimeState *state,
    const char *value)
{
    if (state == NULL ||
        value == NULL) {

        return 0;
    }

    RuntimeValue *runtime_value =
        &state->value;

    if (runtime_value->type !=
        RUNTIME_VALUE_BRANCH) {

        return 0;
    }

    int inserted = 0;

    RuntimeBranchNode *root =
        branch_insert_node(
            runtime_value->branch_root,
            value,
            &inserted
        );

    if (root == NULL &&
        runtime_value->branch_root == NULL) {

        if (!inserted) {
            return 0;
        }
    }

    runtime_value->branch_root = root;

    if (inserted) {
        runtime_value->branch_count++;
    }

    return inserted;
}


int runtime_branch_contains(
    const RuntimeState *state,
    const char *value)
{
    if (state == NULL ||
        value == NULL) {

        return 0;
    }

    if (state->value.type !=
        RUNTIME_VALUE_BRANCH) {

        return 0;
    }

    return branch_contains_node(
        state->value.branch_root,
        value
    );
}


void runtime_branch_print(
    const RuntimeState *state)
{
    if (state == NULL ||
        state->value.type !=
            RUNTIME_VALUE_BRANCH) {

        return;
    }

    int first = 1;

    printf("{");

    branch_print_inorder(
        state->value.branch_root,
        &first
    );

    printf("}");
}

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

        else if (current->value.type ==
                 RUNTIME_VALUE_BRANCH) {

            printf(" = ");

            runtime_branch_print(
                current
            );
        }

        else if (current->value.item_count > 0) {

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
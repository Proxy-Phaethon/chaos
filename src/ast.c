#include "ast.h"

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

ASTNode *ast_create(
    ASTType type,
    const char *value)
{
    ASTNode *node = malloc(sizeof(ASTNode));

    if (node == NULL) {
        return NULL;
    }

    node->type = type;
    node->value = copy_string(value);

    node->children = NULL;
    node->child_count = 0;
    node->child_capacity = 0;

    return node;
}

void ast_add_child(
    ASTNode *parent,
    ASTNode *child)
{
    if (parent == NULL || child == NULL) {
        return;
    }

    if (parent->child_count >= parent->child_capacity) {
        size_t new_capacity =
            parent->child_capacity == 0
            ? 4
            : parent->child_capacity * 2;

        ASTNode **new_children =
            realloc(
                parent->children,
                new_capacity * sizeof(ASTNode *)
            );

        if (new_children == NULL) {
            return;
        }

        parent->children = new_children;
        parent->child_capacity = new_capacity;
    }

    parent->children[
        parent->child_count++
    ] = child;
}

const char *ast_type_name(ASTType type)
{
    switch (type) {

        case AST_PROGRAM:
            return "PROGRAM";

        case AST_REGISTER:
            return "REGISTER";

        case AST_STATE_DECLARATION:
            return "STATE";

        case AST_STATE_NAME:
            return "NAME";

        case AST_STATE_VALUE:
            return "VALUE";

        case AST_DATA_TYPE:
            return "TYPE";

        case AST_DATA_ITEMS:
            return "ITEMS";

        case AST_LOGIC:
            return "LOGIC";

        case AST_EXPRESSION:
            return "EXPRESSION";

        case AST_CONSTANT:
            return "CONSTANT";

        case AST_IF:
            return "IF";

        case AST_ELSE_IF:
            return "ELSE IF";

        case AST_ELSE:
            return "ELSE";

        case AST_CONTRACT_CALL:
            return "CONTRACT";

        case AST_RESULT:
            return "RESULT";

        case AST_TERMINATE:
            return "TERMINATE";

        case AST_DATA_STRUCTURE_OPERATION:
            return "DATA STRUCTURE OPERATION";

        case AST_PUSH:
            return "PUSH";

        case AST_POP:
            return "POP";

        case AST_TRANSITION:
            return "TRANSITION";

        case AST_CONTEXT:
            return "CONTEXT";

        case AST_RULE:
            return "RULE";

        case AST_EXECUTE:
            return "EXECUTE";

        default:
            return "UNKNOWN";
    }
}

void ast_print(
    const ASTNode *node,
    int depth)
{
    if (node == NULL) {
        return;
    }

    for (int i = 0; i < depth; i++) {
        printf("  ");
    }

    printf(
        "%s",
        ast_type_name(node->type)
    );

    if (node->value != NULL) {
        printf(
            ": %s",
            node->value
        );
    }

    printf("\n");

    for (size_t i = 0;
         i < node->child_count;
         i++) {

        ast_print(
            node->children[i],
            depth + 1
        );
    }
}

void ast_free(ASTNode *node)
{
    if (node == NULL) {
        return;
    }

    for (size_t i = 0;
         i < node->child_count;
         i++) {

        ast_free(
            node->children[i]
        );
    }

    free(node->children);
    free(node->value);
    free(node);
}
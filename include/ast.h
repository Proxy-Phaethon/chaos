#ifndef CHAOS_AST_H
#define CHAOS_AST_H

#include <stddef.h>

typedef enum {
    AST_PROGRAM,

    AST_REGISTER,
    AST_STATE_DECLARATION,
    AST_STATE_NAME,
    AST_STATE_VALUE,
    AST_DATA_TYPE,
    AST_DATA_ITEMS,

    AST_LOGIC,
    AST_EXPRESSION,
    AST_CONSTANT,

    AST_IF,
    AST_ELSE_IF,
    AST_ELSE,

    AST_CONTRACT_CALL,
    AST_RESULT,
    AST_TERMINATE,

    AST_DATA_STRUCTURE_OPERATION,
    AST_PUSH,
    AST_POP,

    AST_TRANSITION,
    AST_CONTEXT,
    AST_RULE,

    AST_LIST,
    AST_QUEUE,
    AST_STACK,
    AST_BRANCH,

    AST_EXECUTE
} ASTType;

typedef enum {
    DATA_TYPE_NONE,
    DATA_TYPE_LIST,
    DATA_TYPE_QUEUE,
    DATA_TYPE_STACK,
    DATA_TYPE_BRANCH
} DataType;

typedef struct ASTNode {
    ASTType type;

    char *value;

    DataType data_type;

    struct ASTNode **children;
    size_t child_count;
    size_t child_capacity;

} ASTNode;

ASTNode *ast_create(
    ASTType type,
    const char *value
);

void ast_add_child(
    ASTNode *parent,
    ASTNode *child
);

void ast_print(
    const ASTNode *node,
    int depth
);

void ast_free(
    ASTNode *node
);

const char *ast_type_name(
    ASTType type
);

#endif
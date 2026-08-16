#ifndef AST_H
#define AST_H

#include <stddef.h>

typedef enum {
    AST_PROGRAM,

    AST_LOGIC,
    AST_IF,
    AST_ELSE_IF,
    AST_ELSE,

    AST_STATE,
    AST_TRANSITION,

    AST_CONTEXT,
    AST_RULE,

    AST_EXECUTE,
    AST_CONTRACT_CALL,

    AST_RESULT,
    AST_TERMINATE,

    AST_EXPRESSION
} ASTType;

typedef struct ASTNode {
    ASTType type;

    char *value;

    struct ASTNode **children;
    size_t child_count;
} ASTNode;

ASTNode *ast_create(ASTType type, const char *value);
void ast_add_child(ASTNode *parent, ASTNode *child);
void ast_print(const ASTNode *node, int depth);
void ast_free(ASTNode *node);

#endif
#ifndef CHAOS_PARSER_H
#define CHAOS_PARSER_H

#include "ast.h"
#include "lexer.h"

typedef struct {
    TokenList *tokens;
    size_t current;

    int had_error;
} Parser;

ASTNode *parser_parse(TokenList *tokens);

#endif
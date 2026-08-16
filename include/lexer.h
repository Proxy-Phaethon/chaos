#ifndef LEXER_H
#define LEXER_H

#include <stddef.h>

typedef enum {
    TOKEN_EOF,

    TOKEN_IDENTIFIER,
    TOKEN_EXPRESSION,

    TOKEN_LOGIC,
    TOKEN_IF,
    TOKEN_ELSE,
    TOKEN_STATE,
    TOKEN_TRANSITION,
    TOKEN_CONTEXT,
    TOKEN_RULE,
    TOKEN_EXECUTE,
    TOKEN_RESULT,
    TOKEN_TERMINATE,

    TOKEN_LBRACE,
    TOKEN_RBRACE,
    TOKEN_LPAREN,
    TOKEN_RPAREN,

    TOKEN_COLON,
    TOKEN_COMMA,
    TOKEN_SEMICOLON
} TokenType;

typedef struct {
    TokenType type;
    char *value;
} Token;

typedef struct {
    Token *tokens;
    size_t count;
    size_t capacity;
} TokenList;

TokenList *lexer_tokenize(const char *source);
void lexer_free(TokenList *tokens);

#endif
#ifndef CHAOS_LEXER_H
#define CHAOS_LEXER_H

#include <stddef.h>

typedef enum {
    TOKEN_EOF,

    TOKEN_IDENTIFIER,
    TOKEN_NUMBER,
    TOKEN_STRING,
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
    TOKEN_REGISTER,
    TOKEN_CONSTANT,
    TOKEN_PUSH,
    TOKEN_POP,

    TOKEN_LPAREN,
    TOKEN_RPAREN,
    TOKEN_COLON,
    TOKEN_COMMA,
    TOKEN_SEMICOLON,
    TOKEN_EQUALS
} TokenType;

typedef struct {
    TokenType type;
    char *value;

    size_t line;
    size_t column;
} Token;

typedef struct {
    Token *items;

    size_t count;
    size_t capacity;
} TokenList;

TokenList *lexer_tokenize(const char *source);
void lexer_free(TokenList *tokens);

const char *token_type_name(TokenType type);

#endif
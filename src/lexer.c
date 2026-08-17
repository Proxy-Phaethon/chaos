#include "lexer.h"

#include <ctype.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef struct {
    const char *word;
    TokenType type;
} Keyword;

static const Keyword keywords[] = {
    {"logic",      TOKEN_LOGIC},
    {"if",         TOKEN_IF},
    {"else",       TOKEN_ELSE},
    {"state",      TOKEN_STATE},
    {"transition", TOKEN_TRANSITION},
    {"context",    TOKEN_CONTEXT},
    {"rule",       TOKEN_RULE},
    {"execute",    TOKEN_EXECUTE},
    {"result",     TOKEN_RESULT},
    {"terminate",  TOKEN_TERMINATE},
    {"register",   TOKEN_REGISTER},
    {"constant",   TOKEN_CONSTANT},
    {"push",       TOKEN_PUSH},
    {"pop",        TOKEN_POP}
};

static const size_t keyword_count =
    sizeof(keywords) / sizeof(keywords[0]);

static char *copy_range(const char *start, size_t length)
{
    char *result = malloc(length + 1);

    if (result == NULL) {
        return NULL;
    }

    memcpy(result, start, length);
    result[length] = '\0';

    return result;
}

static TokenType keyword_type(const char *word)
{
    for (size_t i = 0; i < keyword_count; i++) {
        if (strcmp(word, keywords[i].word) == 0) {
            return keywords[i].type;
        }
    }

    return TOKEN_IDENTIFIER;
}

static void add_token(
    TokenList *list,
    TokenType type,
    char *value,
    size_t line,
    size_t column)
{
    if (list->count >= list->capacity) {
        size_t new_capacity =
            list->capacity == 0
            ? 16
            : list->capacity * 2;

        Token *new_items =
            realloc(
                list->items,
                new_capacity * sizeof(Token)
            );

        if (new_items == NULL) {
            free(value);
            return;
        }

        list->items = new_items;
        list->capacity = new_capacity;
    }

    list->items[list->count].type = type;
    list->items[list->count].value = value;
    list->items[list->count].line = line;
    list->items[list->count].column = column;

    list->count++;
}

static void skip_whitespace(
    const char **cursor,
    size_t *line,
    size_t *column)
{
    while (**cursor != '\0') {
        if (**cursor == ' ' ||
            **cursor == '\t' ||
            **cursor == '\r') {

            (*cursor)++;
            (*column)++;
        }
        else if (**cursor == '\n') {
            (*cursor)++;
            (*line)++;
            *column = 1;
        }
        else {
            break;
        }
    }
}

static void lex_identifier(
    const char **cursor,
    TokenList *list,
    size_t *line,
    size_t *column)
{
    const char *start = *cursor;
    size_t start_column = *column;

    while (isalnum((unsigned char)**cursor) ||
           **cursor == '_' ||
           **cursor == '-') {

        (*cursor)++;
        (*column)++;
    }

    size_t length = (size_t)(*cursor - start);

    char *value = copy_range(start, length);

    if (value == NULL) {
        return;
    }

    TokenType type = keyword_type(value);

    add_token(
        list,
        type,
        value,
        *line,
        start_column
    );
}

static void lex_number(
    const char **cursor,
    TokenList *list,
    size_t *line,
    size_t *column)
{
    const char *start = *cursor;
    size_t start_column = *column;

    while (isdigit((unsigned char)**cursor) ||
           **cursor == '.') {

        (*cursor)++;
        (*column)++;
    }

    size_t length = (size_t)(*cursor - start);

    char *value = copy_range(start, length);

    if (value == NULL) {
        return;
    }

    add_token(
        list,
        TOKEN_NUMBER,
        value,
        *line,
        start_column
    );
}

static void lex_string(
    const char **cursor,
    TokenList *list,
    size_t *line,
    size_t *column)
{
    size_t start_column = *column;

    (*cursor)++;
    (*column)++;

    const char *start = *cursor;

    while (**cursor != '\0' &&
           **cursor != '\'') {

        if (**cursor == '\n') {
            (*line)++;
            *column = 1;
        }
        else {
            (*column)++;
        }

        (*cursor)++;
    }

    size_t length = (size_t)(*cursor - start);

    char *value = copy_range(start, length);

    if (value == NULL) {
        return;
    }

    add_token(
        list,
        TOKEN_STRING,
        value,
        *line,
        start_column
    );

    if (**cursor == '\'') {
        (*cursor)++;
        (*column)++;
    }
}

static void lex_expression(
    const char **cursor,
    TokenList *list,
    size_t *line,
    size_t *column)
{
    size_t start_column = *column;

    (*cursor)++;
    (*column)++;

    const char *start = *cursor;

    int depth = 1;

    while (**cursor != '\0' && depth > 0) {
        if (**cursor == '{') {
            depth++;
        }
        else if (**cursor == '}') {
            depth--;

            if (depth == 0) {
                break;
            }
        }

        if (**cursor == '\n') {
            (*line)++;
            *column = 1;
        }
        else {
            (*column)++;
        }

        (*cursor)++;
    }

    size_t length = (size_t)(*cursor - start);

    while (length > 0 &&
           isspace((unsigned char)start[length - 1])) {
        length--;
    }

    char *value = copy_range(start, length);

    if (value == NULL) {
        return;
    }

    add_token(
        list,
        TOKEN_EXPRESSION,
        value,
        *line,
        start_column
    );

    if (**cursor == '}') {
        (*cursor)++;
        (*column)++;
    }
}

static void lex_symbol(
    const char **cursor,
    TokenList *list,
    size_t *line,
    size_t *column)
{
    const char *start = *cursor;
    size_t start_column = *column;

    /*
     * Keep consuming operator characters as one token.
     *
     * Examples:
     * <
     * >
     * <=
     * >=
     * !=
     * +
     * -
     * *
     * /
     */
    while (**cursor != '\0' &&
           strchr("<>!+-*/%&|", **cursor) != NULL) {

        (*cursor)++;
        (*column)++;
    }

    size_t length = (size_t)(*cursor - start);

    char *value = copy_range(start, length);

    if (value == NULL) {
        return;
    }

    add_token(
        list,
        TOKEN_SYMBOL,
        value,
        *line,
        start_column
    );
}

TokenList *lexer_tokenize(const char *source)
{
    if (source == NULL) {
        return NULL;
    }

    TokenList *list = calloc(1, sizeof(TokenList));

    if (list == NULL) {
        return NULL;
    }

    const char *cursor = source;

    size_t line = 1;
    size_t column = 1;

    while (*cursor != '\0') {
        skip_whitespace(
            &cursor,
            &line,
            &column
        );

        if (*cursor == '\0') {
            break;
        }

        size_t token_column = column;

        if (isalpha((unsigned char)*cursor) ||
            *cursor == '_') {

            lex_identifier(
                &cursor,
                list,
                &line,
                &column
            );

            continue;
        }

        if (isdigit((unsigned char)*cursor)) {
            lex_number(
                &cursor,
                list,
                &line,
                &column
            );

            continue;
        }

        if (*cursor == '\'') {
            lex_string(
                &cursor,
                list,
                &line,
                &column
            );

            continue;
        }

        if (*cursor == '{') {
            lex_expression(
                &cursor,
                list,
                &line,
                &column
            );

            continue;
        }

        if (strchr("<>!+-*/%&|", *cursor) != NULL) {
            lex_symbol(
                &cursor,
                list,
                &line,
                &column
            );

            continue;
        }

        TokenType type;

        switch (*cursor) {
            case '(':
                type = TOKEN_LPAREN;
                break;

            case ')':
                type = TOKEN_RPAREN;
                break;

            case ':':
                type = TOKEN_COLON;
                break;

            case ',':
                type = TOKEN_COMMA;
                break;

            case ';':
                type = TOKEN_SEMICOLON;
                break;

            case '=':
                type = TOKEN_EQUALS;
                break;

            default:
                fprintf(
                    stderr,
                    "Lexer error at %zu:%zu: "
                    "unexpected character '%c'\n",
                    line,
                    column,
                    *cursor
                );

                cursor++;
                column++;
                continue;
        }

        add_token(
            list,
            type,
            NULL,
            line,
            token_column
        );

        cursor++;
        column++;
    }

    add_token(
        list,
        TOKEN_EOF,
        NULL,
        line,
        column
    );

    return list;
}

void lexer_free(TokenList *tokens)
{
    if (tokens == NULL) {
        return;
    }

    for (size_t i = 0; i < tokens->count; i++) {
        free(tokens->items[i].value);
    }

    free(tokens->items);
    free(tokens);
}

const char *token_type_name(TokenType type)
{
    switch (type) {
        case TOKEN_EOF:         return "EOF";
        case TOKEN_IDENTIFIER:  return "IDENTIFIER";
        case TOKEN_SYMBOL:      return "SYMBOL";
        case TOKEN_NUMBER:      return "NUMBER";
        case TOKEN_STRING:      return "STRING";
        case TOKEN_EXPRESSION:  return "EXPRESSION";

        case TOKEN_LOGIC:       return "LOGIC";
        case TOKEN_IF:          return "IF";
        case TOKEN_ELSE:        return "ELSE";
        case TOKEN_STATE:       return "STATE";
        case TOKEN_TRANSITION:  return "TRANSITION";
        case TOKEN_CONTEXT:     return "CONTEXT";
        case TOKEN_RULE:        return "RULE";
        case TOKEN_EXECUTE:     return "EXECUTE";
        case TOKEN_RESULT:      return "RESULT";
        case TOKEN_TERMINATE:   return "TERMINATE";
        case TOKEN_REGISTER:    return "REGISTER";
        case TOKEN_CONSTANT:    return "CONSTANT";
        case TOKEN_PUSH:        return "PUSH";
        case TOKEN_POP:         return "POP";

        case TOKEN_LPAREN:      return "LPAREN";
        case TOKEN_RPAREN:      return "RPAREN";
        case TOKEN_COLON:       return "COLON";
        case TOKEN_COMMA:       return "COMMA";
        case TOKEN_SEMICOLON:   return "SEMICOLON";
        case TOKEN_EQUALS:      return "EQUALS";

        default:                return "UNKNOWN";
    }
}
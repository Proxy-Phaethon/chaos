#include "parser.h"

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

static Token *current(Parser *parser)
{
    return &parser->tokens->items[parser->current];
}

static Token *previous(Parser *parser)
{
    return &parser->tokens->items[parser->current - 1];
}

static int check(Parser *parser, TokenType type)
{
    return current(parser)->type == type;
}

static Token *advance_parser(Parser *parser)
{
    if (!check(parser, TOKEN_EOF)) {
        parser->current++;
    }

    return previous(parser);
}

static int match(Parser *parser, TokenType type)
{
    if (!check(parser, type)) {
        return 0;
    }

    advance_parser(parser);
    return 1;
}

static void parser_error(Parser *parser, const char *message)
{
    Token *token = current(parser);

    fprintf(
        stderr,
        "Parser error at %zu:%zu: %s",
        token->line,
        token->column,
        message
    );

    if (token->value != NULL) {
        fprintf(stderr, " near '%s'", token->value);
    }

    fprintf(stderr, "\n");

    parser->had_error = 1;
}

static int expect(
    Parser *parser,
    TokenType type,
    const char *message)
{
    if (check(parser, type)) {
        advance_parser(parser);
        return 1;
    }

    parser_error(parser, message);
    return 0;
}

static ASTNode *parse_condition(Parser *parser)
{
    if (check(parser, TOKEN_EXPRESSION)) {
        Token *token = advance_parser(parser);

        return ast_create(
            AST_EXPRESSION,
            token->value
        );
    }

    if (check(parser, TOKEN_IDENTIFIER)) {
        Token *token = advance_parser(parser);

        return ast_create(
            AST_EXPRESSION,
            token->value
        );
    }

    if (check(parser, TOKEN_RESULT)) {
        Token *token = advance_parser(parser);

        return ast_create(
            AST_RESULT,
            token->value
        );
    }

    parser_error(parser, "expected condition");

    return NULL;
}

static ASTNode *parse_contract_call(Parser *parser)
{
    if (!expect(
            parser,
            TOKEN_LPAREN,
            "expected '(' before contract call")) {
        return NULL;
    }

    if (!check(parser, TOKEN_STRING) &&
        !check(parser, TOKEN_IDENTIFIER)) {

        parser_error(
            parser,
            "expected contract name"
        );

        return NULL;
    }

    Token *contract = advance_parser(parser);

    ASTNode *node = ast_create(
        AST_CONTRACT_CALL,
        contract->value
    );

    if (node == NULL) {
        return NULL;
    }

    while (!check(parser, TOKEN_RPAREN) &&
           !check(parser, TOKEN_EOF)) {

        if (check(parser, TOKEN_RESULT)) {
            Token *argument = advance_parser(parser);

            ASTNode *arg = ast_create(
                AST_RESULT,
                argument->value
            );

            ast_add_child(node, arg);
        }
        else if (check(parser, TOKEN_IDENTIFIER) ||
                 check(parser, TOKEN_STRING)) {

            Token *argument = advance_parser(parser);

            ASTNode *arg = ast_create(
                AST_EXPRESSION,
                argument->value
            );

            ast_add_child(node, arg);
        }
        else {
            parser_error(
                parser,
                "unexpected token inside contract call"
            );

            break;
        }
    }

    if (!expect(
            parser,
            TOKEN_RPAREN,
            "expected ')' after contract call")) {

        ast_free(node);
        return NULL;
    }

    return node;
}

static ASTNode *parse_operation(Parser *parser)
{
    if (check(parser, TOKEN_TERMINATE)) {
        advance_parser(parser);

        return ast_create(
            AST_TERMINATE,
            "terminate"
        );
    }

    return parse_contract_call(parser);
}

static ASTNode *parse_if(Parser *parser)
{
    if (!expect(
            parser,
            TOKEN_IF,
            "expected 'if'")) {
        return NULL;
    }

    ASTNode *node = ast_create(AST_IF, NULL);

    if (node == NULL) {
        return NULL;
    }

    ASTNode *condition = parse_condition(parser);

    if (condition == NULL) {
        ast_free(node);
        return NULL;
    }

    ast_add_child(node, condition);

    if (!expect(
            parser,
            TOKEN_COMMA,
            "expected ',' after condition")) {

        ast_free(node);
        return NULL;
    }

    ASTNode *operation = parse_operation(parser);

    if (operation == NULL) {
        ast_free(node);
        return NULL;
    }

    ast_add_child(node, operation);

    return node;
}

static ASTNode *parse_else_if(Parser *parser)
{
    if (!expect(
            parser,
            TOKEN_ELSE,
            "expected 'else'")) {
        return NULL;
    }

    if (!expect(
            parser,
            TOKEN_IF,
            "expected 'if' after 'else'")) {
        return NULL;
    }

    ASTNode *node = ast_create(AST_ELSE_IF, NULL);

    if (node == NULL) {
        return NULL;
    }

    ASTNode *condition = parse_condition(parser);

    if (condition == NULL) {
        ast_free(node);
        return NULL;
    }

    ast_add_child(node, condition);

    if (!expect(
            parser,
            TOKEN_COMMA,
            "expected ',' after condition")) {

        ast_free(node);
        return NULL;
    }

    ASTNode *operation = parse_operation(parser);

    if (operation == NULL) {
        ast_free(node);
        return NULL;
    }

    ast_add_child(node, operation);

    return node;
}

static ASTNode *parse_else(Parser *parser)
{
    if (!expect(
            parser,
            TOKEN_ELSE,
            "expected 'else'")) {
        return NULL;
    }

    ASTNode *node = ast_create(AST_ELSE, NULL);

    if (node == NULL) {
        return NULL;
    }

    ASTNode *operation = parse_operation(parser);

    if (operation == NULL) {
        ast_free(node);
        return NULL;
    }

    ast_add_child(node, operation);

    return node;
}

static ASTNode *parse_state(Parser *parser)
{
    if (!expect(
            parser,
            TOKEN_STATE,
            "expected 'state'")) {
        return NULL;
    }

    if (!expect(
            parser,
            TOKEN_COLON,
            "expected ':' after 'state'")) {
        return NULL;
    }

    if (!check(parser, TOKEN_IDENTIFIER)) {
        parser_error(
            parser,
            "expected state name"
        );

        return NULL;
    }

    Token *state_name = advance_parser(parser);

    ASTNode *node = ast_create(
        AST_STATE,
        state_name->value
    );

    if (node == NULL) {
        return NULL;
    }

    if (!expect(
            parser,
            TOKEN_COMMA,
            "expected ',' after state")) {

        ast_free(node);
        return NULL;
    }

    if (!expect(
            parser,
            TOKEN_TRANSITION,
            "expected 'transition' after state")) {

        ast_free(node);
        return NULL;
    }

    if (!expect(
            parser,
            TOKEN_LPAREN,
            "expected '(' after transition")) {

        ast_free(node);
        return NULL;
    }

    if (!check(parser, TOKEN_STRING) &&
        !check(parser, TOKEN_IDENTIFIER)) {

        parser_error(
            parser,
            "expected transition reference"
        );

        ast_free(node);
        return NULL;
    }

    Token *transition = advance_parser(parser);

    ASTNode *transition_node = ast_create(
        AST_TRANSITION,
        transition->value
    );

    ast_add_child(node, transition_node);

    if (!expect(
            parser,
            TOKEN_RPAREN,
            "expected ')' after transition")) {

        ast_free(node);
        return NULL;
    }

    return node;
}

static ASTNode *parse_context(Parser *parser)
{
    if (!expect(
            parser,
            TOKEN_CONTEXT,
            "expected 'context'")) {
        return NULL;
    }

    ASTNode *node = ast_create(
        AST_CONTEXT,
        NULL
    );

    if (node == NULL) {
        return NULL;
    }

    ASTNode *expression = parse_condition(parser);

    if (expression == NULL) {
        ast_free(node);
        return NULL;
    }

    ast_add_child(node, expression);

    if (!expect(
            parser,
            TOKEN_COMMA,
            "expected ',' after context")) {

        ast_free(node);
        return NULL;
    }

    if (!expect(
            parser,
            TOKEN_RULE,
            "expected 'rule' after context")) {

        ast_free(node);
        return NULL;
    }

    if (!expect(
            parser,
            TOKEN_LPAREN,
            "expected '(' after rule")) {

        ast_free(node);
        return NULL;
    }

    ASTNode *rule_expression = parse_condition(parser);

    if (rule_expression == NULL) {
        ast_free(node);
        return NULL;
    }

    ASTNode *rule = ast_create(
        AST_RULE,
        NULL
    );

    ast_add_child(rule, rule_expression);
    ast_add_child(node, rule);

    if (!expect(
            parser,
            TOKEN_RPAREN,
            "expected ')' after rule")) {

        ast_free(node);
        return NULL;
    }

    return node;
}

static ASTNode *parse_logic(Parser *parser)
{
    if (!expect(
            parser,
            TOKEN_LOGIC,
            "expected 'logic'")) {
        return NULL;
    }

    ASTNode *logic = ast_create(
        AST_LOGIC,
        NULL
    );

    if (logic == NULL) {
        return NULL;
    }

    ASTNode *condition = parse_condition(parser);

    if (condition == NULL) {
        ast_free(logic);
        return NULL;
    }

    ast_add_child(logic, condition);

    if (!expect(
            parser,
            TOKEN_SEMICOLON,
            "expected ';' after logic condition")) {

        ast_free(logic);
        return NULL;
    }

    while (!check(parser, TOKEN_EXECUTE) &&
           !check(parser, TOKEN_EOF)) {

        ASTNode *statement = NULL;

        if (check(parser, TOKEN_IF)) {
            statement = parse_if(parser);

            while (statement != NULL &&
                   check(parser, TOKEN_ELSE)) {

                /*
                 * Look ahead to distinguish:
                 *
                 * else if ...
                 *
                 * from:
                 *
                 * else ...
                 */
                if (parser->current + 1 <
                    parser->tokens->count &&
                    parser->tokens->items[
                        parser->current + 1
                    ].type == TOKEN_IF) {

                    ASTNode *else_if =
                        parse_else_if(parser);

                    if (else_if != NULL) {
                        ast_add_child(logic, else_if);
                    }
                }
                else {
                    ASTNode *else_node =
                        parse_else(parser);

                    if (else_node != NULL) {
                        ast_add_child(logic, else_node);
                    }

                    break;
                }
            }
        }
        else if (check(parser, TOKEN_STATE)) {
            statement = parse_state(parser);
        }
        else if (check(parser, TOKEN_CONTEXT)) {
            statement = parse_context(parser);
        }
        else {
            parser_error(
                parser,
                "unexpected statement inside logic"
            );

            advance_parser(parser);
            continue;
        }

        if (statement != NULL) {
            ast_add_child(logic, statement);
        }

        if (check(parser, TOKEN_SEMICOLON)) {
            advance_parser(parser);
        }
    }

    return logic;
}

static ASTNode *parse_execute(Parser *parser)
{
    if (!expect(
            parser,
            TOKEN_EXECUTE,
            "expected 'execute'")) {
        return NULL;
    }

    return ast_create(
        AST_EXECUTE,
        "execute"
    );
}

ASTNode *parser_parse(TokenList *tokens)
{
    if (tokens == NULL || tokens->count == 0) {
        return NULL;
    }

    Parser parser = {
        .tokens = tokens,
        .current = 0,
        .had_error = 0
    };

    ASTNode *program = ast_create(
        AST_PROGRAM,
        NULL
    );

    if (program == NULL) {
        return NULL;
    }

    while (!check(&parser, TOKEN_EOF)) {
        if (check(&parser, TOKEN_LOGIC)) {
            ASTNode *logic = parse_logic(&parser);

            if (logic != NULL) {
                ast_add_child(program, logic);
            }
        }
        else if (check(&parser, TOKEN_EXECUTE)) {
            ASTNode *execute = parse_execute(&parser);

            if (execute != NULL) {
                ast_add_child(program, execute);
            }
        }
        else {
            parser_error(
                &parser,
                "expected 'logic' or 'execute'"
            );

            advance_parser(&parser);
        }
    }

    if (parser.had_error) {
        ast_free(program);
        return NULL;
    }

    return program;
}
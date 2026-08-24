#include "parser.h"

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

static Token *current(Parser *parser)
{
    return &parser->tokens->items[parser->current];
}

static Token *advance_parser(Parser *parser)
{
    if (current(parser)->type != TOKEN_EOF) {
        parser->current++;
    }

    return &parser->tokens->items[parser->current - 1];
}

static int check(Parser *parser, TokenType type)
{
    return current(parser)->type == type;
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

/*
 * Parse a value used by states and push operations.
 */
static ASTNode *parse_value(Parser *parser)
{
    if (check(parser, TOKEN_EXPRESSION) ||
        check(parser, TOKEN_STRING) ||
        check(parser, TOKEN_NUMBER) ||
        check(parser, TOKEN_IDENTIFIER)) {

        Token *token = advance_parser(parser);

        return ast_create(
            AST_STATE_VALUE,
            token->value
        );
    }

    parser_error(parser, "expected value");

    return NULL;
}

/*
 * Parse:
 *
 *     state: x = 3
 *
 * or:
 *
 *     state: fruits, list = {'apple', 'banana'}
 */
static ASTNode *parse_register_state(Parser *parser)
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

    Token *name = advance_parser(parser);

    ASTNode *state = ast_create(
        AST_STATE_DECLARATION,
        NULL
    );

    if (state == NULL) {
        return NULL;
    }

    ASTNode *name_node = ast_create(
        AST_STATE_NAME,
        name->value
    );

    if (name_node == NULL) {
        ast_free(state);
        return NULL;
    }

    ast_add_child(state, name_node);

    /*
     * Ordinary state:
     *
     *     state: x = 42
     */
    if (check(parser, TOKEN_EQUALS)) {
        advance_parser(parser);

        ASTNode *value = parse_value(parser);

        if (value == NULL) {
            ast_free(state);
            return NULL;
        }

        ast_add_child(state, value);

        return state;
    }

    /*
     * Data structure state:
     *
     *     state: fruits, list = {...}
     */
    if (check(parser, TOKEN_COMMA)) {
        advance_parser(parser);

        if (!check(parser, TOKEN_LIST) &&
            !check(parser, TOKEN_QUEUE) &&
            !check(parser, TOKEN_STACK) &&
            !check(parser, TOKEN_BRANCH)) {

            parser_error(
                parser,
                "expected data structure type"
            );

            ast_free(state);
            return NULL;
        }

        Token *type = advance_parser(parser);

        ASTNode *type_node = ast_create(
            AST_DATA_TYPE,
            type->value
        );

        if (type_node == NULL) {
            ast_free(state);
            return NULL;
        }

        ast_add_child(state, type_node);

        if (!expect(
                parser,
                TOKEN_EQUALS,
                "expected '=' after data structure type")) {

            ast_free(state);
            return NULL;
        }

        ASTNode *value = parse_value(parser);

        if (value == NULL) {
            ast_free(state);
            return NULL;
        }

        ast_add_child(state, value);

        return state;
    }

    /*
     * A state containing only a name is valid.
     */
    return state;
}

/*
 * Parse:
 *
 *     register ('name'):
 *
 *         state: ...
 *         state: ...
 *         ...
 *
 *     ;
 */
static ASTNode *parse_register(Parser *parser)
{
    if (!expect(
            parser,
            TOKEN_REGISTER,
            "expected 'register'")) {

        return NULL;
    }

    ASTNode *register_node = ast_create(
        AST_REGISTER,
        NULL
    );

    if (register_node == NULL) {
        return NULL;
    }

    /*
     * Optional register name.
     */
    if (check(parser, TOKEN_LPAREN)) {
        advance_parser(parser);

        if (!check(parser, TOKEN_STRING) &&
            !check(parser, TOKEN_IDENTIFIER)) {

            parser_error(
                parser,
                "expected register name"
            );

            ast_free(register_node);
            return NULL;
        }

        Token *name = advance_parser(parser);

        register_node->value = malloc(
            strlen(name->value) + 1
        );

        if (register_node->value == NULL) {
            ast_free(register_node);
            return NULL;
        }

        strcpy(
            register_node->value,
            name->value
        );

        if (!expect(
                parser,
                TOKEN_RPAREN,
                "expected ')' after register name")) {

            ast_free(register_node);
            return NULL;
        }
    }

    if (check(parser, TOKEN_COLON)) {
        advance_parser(parser);
    }

    /*
     * Parse states until ';'.
     */
    while (!check(parser, TOKEN_SEMICOLON) &&
           !check(parser, TOKEN_EOF)) {

        ASTNode *state =
            parse_register_state(parser);

        if (state == NULL) {
            ast_free(register_node);
            return NULL;
        }

        ast_add_child(
            register_node,
            state
        );

        if (check(parser, TOKEN_COMMA)) {
            advance_parser(parser);
            continue;
        }

        break;
    }

    if (!expect(
            parser,
            TOKEN_SEMICOLON,
            "expected ';' at end of register")) {

        ast_free(register_node);
        return NULL;
    }

    return register_node;
}

/*
 * Parse:
 *
 *     constant: x < y;
 *
 * Everything between ':' and ';' becomes
 * the constant expression.
 */
static ASTNode *parse_constant(Parser *parser)
{
    if (!expect(
            parser,
            TOKEN_CONSTANT,
            "expected 'constant'")) {

        return NULL;
    }

    if (!expect(
            parser,
            TOKEN_COLON,
            "expected ':' after 'constant'")) {

        return NULL;
    }

    ASTNode *constant = ast_create(
        AST_CONSTANT,
        NULL
    );

    if (constant == NULL) {
        return NULL;
    }

    char buffer[4096];
    buffer[0] = '\0';

    while (!check(parser, TOKEN_SEMICOLON) &&
           !check(parser, TOKEN_EOF)) {

        Token *token = advance_parser(parser);

        if (token->value != NULL) {
            if (buffer[0] != '\0') {
                strcat(buffer, " ");
            }

            strcat(buffer, token->value);
        }
    }

    ASTNode *value = ast_create(
        AST_STATE_VALUE,
        buffer
    );

    if (value == NULL) {
        ast_free(constant);
        return NULL;
    }

    ast_add_child(
        constant,
        value
    );

    if (!expect(
            parser,
            TOKEN_SEMICOLON,
            "expected ';' after constant")) {

        ast_free(constant);
        return NULL;
    }

    return constant;
}

/*
 * Parse:
 *
 *     list fruits
 *         (push 'apple')
 *         (pop),
 *
 *     queue waiting
 *         (push 'fourth')
 *         (pop),
 *
 *     stack history
 *         ...
 *
 *     branch tree
 *         ...
 */
static ASTNode *parse_data_structure_operation(
    Parser *parser)
{
    ASTType data_type;

    /*
     * Determine the data structure type.
     */
    if (check(parser, TOKEN_LIST)) {
        data_type = AST_LIST;
    }
    else if (check(parser, TOKEN_QUEUE)) {
        data_type = AST_QUEUE;
    }
    else if (check(parser, TOKEN_STACK)) {
        data_type = AST_STACK;
    }
    else if (check(parser, TOKEN_BRANCH)) {
        data_type = AST_BRANCH;
    }
    else {
        parser_error(
            parser,
            "expected data structure type"
        );

        return NULL;
    }

    Token *type_token = advance_parser(parser);

    /*
     * Every data structure operation must name
     * the structure it operates on.
     */
    if (!check(parser, TOKEN_IDENTIFIER)) {
        parser_error(
            parser,
            "expected data structure name"
        );

        return NULL;
    }

    Token *name_token = advance_parser(parser);

    ASTNode *node = ast_create(
        AST_DATA_STRUCTURE_OPERATION,
        name_token->value
    );

    if (node == NULL) {
        return NULL;
    }

    /*
     * Store the data structure type as the
     * first child.
     */
    ASTNode *type_node = ast_create(
        data_type,
        type_token->value
    );

    if (type_node == NULL) {
        ast_free(node);
        return NULL;
    }

    ast_add_child(
        node,
        type_node
    );

    /*
     * Parse all operations belonging to this
     * data structure.
     *
     * Example:
     *
     *     list fruits
     *         (push 'apple')
     *         (push 'banana')
     *         (pop),
     *
     * The comma terminates the group.
     */
    while (!check(parser, TOKEN_COMMA) &&
           !check(parser, TOKEN_EOF) &&
           !check(parser, TOKEN_SEMICOLON)) {

        if (!expect(
                parser,
                TOKEN_LPAREN,
                "expected '(' before data structure operation")) {

            ast_free(node);
            return NULL;
        }

        ASTNode *operation = NULL;

        /*
         * push
         */
        if (check(parser, TOKEN_PUSH)) {
            advance_parser(parser);

            operation = ast_create(
                AST_PUSH,
                NULL
            );

            if (operation == NULL) {
                ast_free(node);
                return NULL;
            }

            ASTNode *value = parse_value(parser);

            if (value == NULL) {
                ast_free(operation);
                ast_free(node);
                return NULL;
            }

            ast_add_child(
                operation,
                value
            );
        }

        /*
         * pop
         */
        else if (check(parser, TOKEN_POP)) {
            advance_parser(parser);

            operation = ast_create(
                AST_POP,
                NULL
            );

            if (operation == NULL) {
                ast_free(node);
                return NULL;
            }
        }

        else {
            parser_error(
                parser,
                "expected 'push' or 'pop'"
            );

            ast_free(node);
            return NULL;
        }

        /*
         * Every operation has its own parentheses.
         */
        if (!expect(
                parser,
                TOKEN_RPAREN,
                "expected ')' after data structure operation")) {

            ast_free(operation);
            ast_free(node);
            return NULL;
        }

        ast_add_child(
            node,
            operation
        );
    }

    /*
     * The comma belongs to the data structure
     * group and terminates it.
     */
    if (!expect(
            parser,
            TOKEN_COMMA,
            "expected ',' after data structure operation")) {

        ast_free(node);
        return NULL;
    }

    return node;
}

/*
 * Parse a contract call:
 *
 *     (contract ...)
 */
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

            if (arg == NULL) {
                ast_free(node);
                return NULL;
            }

            ast_add_child(node, arg);
        }
        else if (check(parser, TOKEN_IDENTIFIER) ||
                 check(parser, TOKEN_STRING) ||
                 check(parser, TOKEN_NUMBER)) {

            Token *argument = advance_parser(parser);

            ASTNode *arg = ast_create(
                AST_EXPRESSION,
                argument->value
            );

            if (arg == NULL) {
                ast_free(node);
                return NULL;
            }

            ast_add_child(node, arg);
        }
        else {
            parser_error(
                parser,
                "unexpected token inside contract call"
            );

            ast_free(node);
            return NULL;
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

/*
 * Parse a condition/expression.
 */
static ASTNode *parse_condition(Parser *parser)
{
    if (check(parser, TOKEN_EXPRESSION)) {
        Token *token = advance_parser(parser);

        return ast_create(
            AST_EXPRESSION,
            token->value
        );
    }

    if (check(parser, TOKEN_STRING) ||
        check(parser, TOKEN_IDENTIFIER) ||
        check(parser, TOKEN_NUMBER)) {

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

    parser_error(
        parser,
        "expected condition"
    );

    return NULL;
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

    ASTNode *node = ast_create(
        AST_IF,
        NULL
    );

    if (node == NULL) {
        return NULL;
    }

    ASTNode *condition =
        parse_condition(parser);

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

    ASTNode *operation =
        parse_operation(parser);

    if (operation == NULL) {
        ast_free(node);
        return NULL;
    }

    ast_add_child(node, operation);

    return node;
}

static ASTNode *parse_else_if(Parser *parser)
{
    advance_parser(parser);
    advance_parser(parser);

    ASTNode *node = ast_create(
        AST_ELSE_IF,
        NULL
    );

    if (node == NULL) {
        return NULL;
    }

    ASTNode *condition =
        parse_condition(parser);

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

    ASTNode *operation =
        parse_operation(parser);

    if (operation == NULL) {
        ast_free(node);
        return NULL;
    }

    ast_add_child(node, operation);

    return node;
}

static ASTNode *parse_else(Parser *parser)
{
    advance_parser(parser);

    ASTNode *node = ast_create(
        AST_ELSE,
        NULL
    );

    if (node == NULL) {
        return NULL;
    }

    ASTNode *operation =
        parse_operation(parser);

    if (operation == NULL) {
        ast_free(node);
        return NULL;
    }

    ast_add_child(node, operation);

    return node;
}

static ASTNode *parse_transition(Parser *parser)
{
    if (!expect(
            parser,
            TOKEN_TRANSITION,
            "expected 'transition'")) {

        return NULL;
    }

    if (!expect(
            parser,
            TOKEN_LPAREN,
            "expected '(' after transition")) {

        return NULL;
    }

    if (!check(parser, TOKEN_STRING) &&
        !check(parser, TOKEN_IDENTIFIER)) {

        parser_error(
            parser,
            "expected transition reference"
        );

        return NULL;
    }

    Token *transition = advance_parser(parser);

    ASTNode *node = ast_create(
        AST_TRANSITION,
        transition->value
    );

    if (node == NULL) {
        return NULL;
    }

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

    ASTNode *expression =
        parse_condition(parser);

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

    ASTNode *rule_expression =
        parse_condition(parser);

    if (rule_expression == NULL) {
        ast_free(node);
        return NULL;
    }

    ASTNode *rule = ast_create(
        AST_RULE,
        NULL
    );

    if (rule == NULL) {
        ast_free(rule_expression);
        ast_free(node);
        return NULL;
    }

    ast_add_child(
        rule,
        rule_expression
    );

    ast_add_child(
        node,
        rule
    );

    if (!expect(
            parser,
            TOKEN_RPAREN,
            "expected ')' after rule")) {

        ast_free(node);
        return NULL;
    }

    return node;
}

/*
 * Parse:
 *
 *     logic {x > 0};
 *
 * followed by zero or more logic statements.
 */
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

    ASTNode *question =
        parse_condition(parser);

    if (question == NULL) {
        ast_free(logic);
        return NULL;
    }

    ast_add_child(
        logic,
        question
    );

    if (!expect(
            parser,
            TOKEN_SEMICOLON,
            "expected ';' after logic question")) {

        ast_free(logic);
        return NULL;
    }

    while (!check(parser, TOKEN_EXECUTE) &&
           !check(parser, TOKEN_EOF)) {

        /*
         * if / else-if / else
         */
        if (check(parser, TOKEN_IF)) {
            ASTNode *if_node =
                parse_if(parser);

            if (if_node == NULL) {
                ast_free(logic);
                return NULL;
            }

            ast_add_child(
                logic,
                if_node
            );

            while (check(parser, TOKEN_ELSE)) {

                if (parser->current + 1 <
                    parser->tokens->count &&
                    parser->tokens->items[
                        parser->current + 1
                    ].type == TOKEN_IF) {

                    ASTNode *else_if =
                        parse_else_if(parser);

                    if (else_if == NULL) {
                        ast_free(logic);
                        return NULL;
                    }

                    ast_add_child(
                        logic,
                        else_if
                    );
                }
                else {
                    ASTNode *else_node =
                        parse_else(parser);

                    if (else_node == NULL) {
                        ast_free(logic);
                        return NULL;
                    }

                    ast_add_child(
                        logic,
                        else_node
                    );

                    break;
                }
            }

            if (check(parser, TOKEN_SEMICOLON)) {
                advance_parser(parser);
            }

            continue;
        }

        /*
         * constant
         */
        if (check(parser, TOKEN_CONSTANT)) {
            ASTNode *constant =
                parse_constant(parser);

            if (constant == NULL) {
                ast_free(logic);
                return NULL;
            }

            ast_add_child(
                logic,
                constant
            );

            continue;
        }

        /*
         * Data structure operation group:
         *
         * list fruits
         *     (push 'apple')
         *     (pop),
         */
        if (check(parser, TOKEN_LIST) ||
            check(parser, TOKEN_QUEUE) ||
            check(parser, TOKEN_STACK) ||
            check(parser, TOKEN_BRANCH)) {

            ASTNode *operation =
                parse_data_structure_operation(parser);

            if (operation == NULL) {
                ast_free(logic);
                return NULL;
            }

            ast_add_child(
                logic,
                operation
            );

            continue;
        }

        /*
         * transition
         */
        if (check(parser, TOKEN_TRANSITION)) {
            ASTNode *transition =
                parse_transition(parser);

            if (transition == NULL) {
                ast_free(logic);
                return NULL;
            }

            ast_add_child(
                logic,
                transition
            );

            if (check(parser, TOKEN_SEMICOLON)) {
                advance_parser(parser);
            }

            continue;
        }

        /*
         * context
         */
        if (check(parser, TOKEN_CONTEXT)) {
            ASTNode *context =
                parse_context(parser);

            if (context == NULL) {
                ast_free(logic);
                return NULL;
            }

            ast_add_child(
                logic,
                context
            );

            if (check(parser, TOKEN_SEMICOLON)) {
                advance_parser(parser);
            }

            continue;
        }

        parser_error(
            parser,
            "unexpected statement inside logic"
        );

        advance_parser(parser);
    }

    return logic;
}

static ASTNode *parse_execute(Parser *parser)
{
    advance_parser(parser);

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

        if (check(&parser, TOKEN_REGISTER)) {
            ASTNode *register_node =
                parse_register(&parser);

            if (register_node == NULL) {
                ast_free(program);
                return NULL;
            }

            ast_add_child(
                program,
                register_node
            );

            continue;
        }

        if (check(&parser, TOKEN_LOGIC)) {
            ASTNode *logic =
                parse_logic(&parser);

            if (logic == NULL) {
                ast_free(program);
                return NULL;
            }

            ast_add_child(
                program,
                logic
            );

            continue;
        }

        if (check(&parser, TOKEN_EXECUTE)) {
            ASTNode *execute =
                parse_execute(&parser);

            if (execute == NULL) {
                ast_free(program);
                return NULL;
            }

            ast_add_child(
                program,
                execute
            );

            continue;
        }

        parser_error(
            &parser,
            "expected 'register', 'logic', or 'execute'"
        );

        advance_parser(&parser);
    }

    if (parser.had_error) {
        ast_free(program);
        return NULL;
    }

    return program;
}
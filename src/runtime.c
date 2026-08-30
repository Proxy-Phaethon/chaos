#include "runtime.h"

#include <ctype.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef struct {
    const char *input;
    size_t position;
    RuntimeStateStore *states;
} ExpressionParser;

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

static int is_collection_type(RuntimeValueType type)
{
    return type == RUNTIME_VALUE_LIST ||
           type == RUNTIME_VALUE_QUEUE ||
           type == RUNTIME_VALUE_STACK;
}

static RuntimeValueType runtime_data_type(
    const char *type)
{
    if (type == NULL) {
        return RUNTIME_VALUE_STRING;
    }

    if (strcmp(type, "list") == 0) {
        return RUNTIME_VALUE_LIST;
    }

    if (strcmp(type, "queue") == 0) {
        return RUNTIME_VALUE_QUEUE;
    }

    if (strcmp(type, "stack") == 0) {
        return RUNTIME_VALUE_STACK;
    }

    if (strcmp(type, "branch") == 0) {
        return RUNTIME_VALUE_BRANCH;
    }

    return RUNTIME_VALUE_STRING;
}

static RuntimeValueType runtime_scalar_type(
    const char *value)
{
    if (value == NULL) {
        return RUNTIME_VALUE_STRING;
    }

    char *end = NULL;

    strtod(value, &end);

    if (end != value && *end == '\0') {
        return RUNTIME_VALUE_NUMBER;
    }

    return RUNTIME_VALUE_STRING;
}

static int runtime_value_is_true(
    const RuntimeValue *value)
{
    if (value == NULL) {
        return 0;
    }

    if (value->type == RUNTIME_VALUE_NUMBER) {
        if (value->scalar == NULL) {
            return 0;
        }

        char *end = NULL;
        double number = strtod(
            value->scalar,
            &end
        );

        if (end == value->scalar || *end != '\0') {
            return 0;
        }

        return number != 0.0;
    }

    if (value->scalar == NULL) {
        return 0;
    }

    if (strcmp(value->scalar, "false") == 0 ||
        strcmp(value->scalar, "0") == 0 ||
        strcmp(value->scalar, "") == 0) {

        return 0;
    }

    return 1;
}

static void expression_skip_spaces(
    ExpressionParser *parser)
{
    if (parser == NULL) {
        return;
    }

    while (isspace(
        (unsigned char)parser->input[
            parser->position
        ])) {

        parser->position++;
    }
}

static char expression_current(
    ExpressionParser *parser)
{
    expression_skip_spaces(parser);

    return parser->input[
        parser->position
    ];
}

static int expression_match(
    ExpressionParser *parser,
    const char *operator)
{
    size_t length;

    if (parser == NULL ||
        operator == NULL) {

        return 0;
    }

    expression_skip_spaces(parser);

    length = strlen(operator);

    if (strncmp(
            parser->input +
                parser->position,
            operator,
            length
        ) == 0) {

        parser->position += length;
        return 1;
    }

    return 0;
}

static int expression_parse_logical(
    ExpressionParser *parser,
    double *result
);

static int expression_parse_comparison(
    ExpressionParser *parser,
    double *result
);

static int expression_parse_arithmetic(
    ExpressionParser *parser,
    double *result
);

static int expression_parse_term(
    ExpressionParser *parser,
    double *result
);

static int expression_parse_factor(
    ExpressionParser *parser,
    double *result
);

static int expression_parse_primary(
    ExpressionParser *parser,
    double *result)
{
    expression_skip_spaces(parser);

    char current =
        expression_current(parser);

    if (current == '(') {

        parser->position++;

        if (!expression_parse_logical(
                parser,
                result)) {

            return 0;
        }

        if (!expression_match(
                parser,
                ")")) {

            fprintf(
                stderr,
                "Runtime error: expected ')' in expression\n"
            );

            return 0;
        }

        return 1;
    }

    if (isdigit((unsigned char)current) ||
        current == '.') {

        char *end = NULL;

        double value =
            strtod(
                parser->input +
                    parser->position,
                &end
            );

        if (end ==
            parser->input +
                parser->position) {

            fprintf(
                stderr,
                "Runtime error: invalid number in expression\n"
            );

            return 0;
        }

        parser->position =
            (size_t)(
                end -
                parser->input
            );

        *result = value;

        return 1;
    }

    if (isalpha((unsigned char)current) ||
        current == '_') {

        char name[256];
        size_t length = 0;

        while (
            isalnum(
                (unsigned char)parser->input[
                    parser->position
                ]
            ) ||
            parser->input[
                parser->position
            ] == '_'
        ) {

            if (length < sizeof(name) - 1) {
                name[length++] =
                    parser->input[
                        parser->position
                    ];
            }

            parser->position++;
        }

        name[length] = '\0';

        if (strcmp(name, "true") == 0) {
            *result = 1.0;
            return 1;
        }

        if (strcmp(name, "false") == 0) {
            *result = 0.0;
            return 1;
        }

        RuntimeState *state =
            runtime_state_find(
                parser->states,
                name
            );

        if (state == NULL) {

            fprintf(
                stderr,
                "Runtime error: unknown state '%s' "
                "in expression\n",
                name
            );

            return 0;
        }

        if (state->value.type !=
            RUNTIME_VALUE_NUMBER) {

            fprintf(
                stderr,
                "Runtime error: state '%s' "
                "is not a number\n",
                name
            );

            return 0;
        }

        if (state->value.scalar == NULL) {

            fprintf(
                stderr,
                "Runtime error: state '%s' "
                "has no numeric value\n",
                name
            );

            return 0;
        }

        char *end = NULL;

        double value =
            strtod(
                state->value.scalar,
                &end
            );

        if (end == state->value.scalar ||
            *end != '\0') {

            fprintf(
                stderr,
                "Runtime error: state '%s' "
                "contains an invalid number\n",
                name
            );

            return 0;
        }

        *result = value;

        return 1;
    }

    fprintf(
        stderr,
        "Runtime error: unexpected character '%c' "
        "in expression\n",
        current
    );

    return 0;
}

static int expression_parse_factor(
    ExpressionParser *parser,
    double *result)
{
    expression_skip_spaces(parser);

    if (expression_match(parser, "+")) {
        return expression_parse_factor(
            parser,
            result
        );
    }

    if (expression_match(parser, "-")) {

        if (!expression_parse_factor(
                parser,
                result)) {

            return 0;
        }

        *result = -*result;

        return 1;
    }

    if (expression_match(parser, "!")) {

        if (!expression_parse_factor(
                parser,
                result)) {

            return 0;
        }

        *result = (*result == 0.0)
            ? 1.0
            : 0.0;

        return 1;
    }

    return expression_parse_primary(
        parser,
        result
    );
}

static int expression_parse_term(
    ExpressionParser *parser,
    double *result)
{
    if (!expression_parse_factor(
            parser,
            result)) {

        return 0;
    }

    while (1) {

        expression_skip_spaces(parser);

        char current =
            expression_current(parser);

        if (current != '*' &&
            current != '/' &&
            current != '%') {

            break;
        }

        parser->position++;

        double right;

        if (!expression_parse_factor(
                parser,
                &right)) {

            return 0;
        }

        if (current == '*') {
            *result *= right;
        }
        else if (current == '/') {

            if (right == 0.0) {

                fprintf(
                    stderr,
                    "Runtime error: division by zero\n"
                );

                return 0;
            }

            *result /= right;
        }
        else {

            if (right == 0.0) {

                fprintf(
                    stderr,
                    "Runtime error: modulo by zero\n"
                );

                return 0;
            }

            long long left_integer =
                (long long)*result;

            long long right_integer =
                (long long)right;

            *result =
                (double)(
                    left_integer %
                    right_integer
                );
        }
    }

    return 1;
}

static int expression_parse_arithmetic(
    ExpressionParser *parser,
    double *result)
{
    if (!expression_parse_term(
            parser,
            result)) {

        return 0;
    }

    while (1) {

        expression_skip_spaces(parser);

        char current =
            expression_current(parser);

        if (current != '+' &&
            current != '-') {

            break;
        }

        parser->position++;

        double right;

        if (!expression_parse_term(
                parser,
                &right)) {

            return 0;
        }

        if (current == '+') {
            *result += right;
        }
        else {
            *result -= right;
        }
    }

    return 1;
}

static int expression_parse_comparison(
    ExpressionParser *parser,
    double *result)
{
    if (!expression_parse_arithmetic(
            parser,
            result)) {

        return 0;
    }

    while (1) {

        expression_skip_spaces(parser);

        const char *input =
            parser->input +
            parser->position;

        const char *operator = NULL;

        if (strncmp(input, "<=", 2) == 0) {
            operator = "<=";
        }
        else if (strncmp(input, ">=", 2) == 0) {
            operator = ">=";
        }
        else if (strncmp(input, "==", 2) == 0) {
            operator = "==";
        }
        else if (strncmp(input, "!=", 2) == 0) {
            operator = "!=";
        }
        else if (*input == '<') {
            operator = "<";
        }
        else if (*input == '>') {
            operator = ">";
        }
        else {
            break;
        }

        parser->position += strlen(operator);

        double left = *result;
        double right;

        if (!expression_parse_arithmetic(
                parser,
                &right)) {

            return 0;
        }

        if (strcmp(operator, "<") == 0) {
            *result = left < right;
        }
        else if (strcmp(operator, ">") == 0) {
            *result = left > right;
        }
        else if (strcmp(operator, "<=") == 0) {
            *result = left <= right;
        }
        else if (strcmp(operator, ">=") == 0) {
            *result = left >= right;
        }
        else if (strcmp(operator, "==") == 0) {
            *result = left == right;
        }
        else {
            *result = left != right;
        }
    }

    return 1;
}

static int expression_parse_logical(
    ExpressionParser *parser,
    double *result)
{
    if (!expression_parse_comparison(
            parser,
            result)) {

        return 0;
    }

    while (1) {

        expression_skip_spaces(parser);

        if (strncmp(
                parser->input +
                    parser->position,
                "&&",
                2
            ) == 0) {

            parser->position += 2;

            double right;

            if (!expression_parse_comparison(
                    parser,
                    &right)) {

                return 0;
            }

            *result =
                (*result != 0.0 &&
                 right != 0.0)
                ? 1.0
                : 0.0;

            continue;
        }

        if (strncmp(
                parser->input +
                    parser->position,
                "||",
                2
            ) == 0) {

            parser->position += 2;

            double right;

            if (!expression_parse_comparison(
                    parser,
                    &right)) {

                return 0;
            }

            *result =
                (*result != 0.0 ||
                 right != 0.0)
                ? 1.0
                : 0.0;

            continue;
        }

        break;
    }

    return 1;
}

static int runtime_evaluate_expression(
    RuntimeStateStore *states,
    const char *expression,
    double *result)
{
    if (states == NULL ||
        expression == NULL ||
        result == NULL) {

        return 0;
    }

    ExpressionParser parser = {
        .input = expression,
        .position = 0,
        .states = states
    };

    if (!expression_parse_logical(
            &parser,
            result)) {

        return 0;
    }

    expression_skip_spaces(&parser);

    if (parser.input[
            parser.position
        ] != '\0') {

        fprintf(
            stderr,
            "Runtime error: unexpected text "
            "in expression near '%s'\n",
            parser.input +
                parser.position
        );

        return 0;
    }

    return 1;
}

Runtime *runtime_create(void)
{
    Runtime *runtime =
        calloc(
            1,
            sizeof(Runtime)
        );

    if (runtime == NULL) {
        return NULL;
    }

    runtime->states =
        runtime_state_store_create();

    if (runtime->states == NULL) {
        free(runtime);
        return NULL;
    }

    runtime->current_state = NULL;

    return runtime;
}

void runtime_free(
    Runtime *runtime)
{
    if (runtime == NULL) {
        return;
    }

    runtime_state_store_free(
        runtime->states
    );

    free(runtime->current_state);
    free(runtime);
}

static int runtime_initialize_collection(
    RuntimeState *state,
    const char *value)
{
    if (state == NULL) {
        return 0;
    }

    if (state->value.type ==
        RUNTIME_VALUE_BRANCH) {

        if (value == NULL ||
            value[0] == '\0') {

            return 1;
        }

        char *contents =
            copy_string(value);

        if (contents == NULL) {
            return 0;
        }

        char *item =
            strtok(contents, ",");

        while (item != NULL) {

            while (isspace(
                (unsigned char)*item)) {

                item++;
            }

            size_t length =
                strlen(item);

            while (
                length > 0 &&
                isspace(
                    (unsigned char)item[
                        length - 1
                    ]
                )
            ) {

                item[length - 1] = '\0';
                length--;
            }

            if (length >= 2 &&
                item[0] == '\'' &&
                item[length - 1] == '\'') {

                item[length - 1] = '\0';
                item++;
            }

            if (!runtime_branch_insert(
                    state,
                    item)) {

                free(contents);

                if (runtime_branch_contains(
                        state,
                        item)) {

                    item = strtok(
                        NULL,
                        ","
                    );

                    continue;
                }

                return 0;
            }

            item = strtok(NULL, ",");
        }

        free(contents);
        return 1;
    }

    if (!is_collection_type(
            state->value.type)) {

        return 1;
    }

    if (value == NULL ||
        value[0] == '\0') {

        return 1;
    }

    char *contents =
        copy_string(value);

    if (contents == NULL) {
        return 0;
    }

    char *item =
        strtok(contents, ",");

    while (item != NULL) {

        while (isspace(
            (unsigned char)*item)) {

            item++;
        }

        size_t length =
            strlen(item);

        while (
            length > 0 &&
            isspace(
                (unsigned char)item[
                    length - 1
                ]
            )
        ) {

            item[length - 1] = '\0';
            length--;
        }

        if (length >= 2 &&
            item[0] == '\'' &&
            item[length - 1] == '\'') {

            item[length - 1] = '\0';
            item++;
        }

        if (!runtime_state_push(
                state,
                item)) {

            free(contents);
            return 0;
        }

        item = strtok(NULL, ",");
    }

    free(contents);

    return 1;
}

RuntimeResult runtime_execute_register(
    Runtime *runtime,
    const ASTNode *register_node)
{
    if (runtime == NULL ||
        register_node == NULL) {

        return RUNTIME_ERROR;
    }

    for (size_t i = 0;
         i < register_node->child_count;
         i++) {

        const ASTNode *state =
            register_node->children[i];

        if (state->type !=
            AST_STATE_DECLARATION) {

            continue;
        }

        const char *name = NULL;
        const char *value = NULL;
        const char *data_type = NULL;

        int is_expression = 0;

        for (size_t j = 0;
             j < state->child_count;
             j++) {

            const ASTNode *child =
                state->children[j];

            switch (child->type) {

                case AST_STATE_NAME:
                    name = child->value;
                    break;

                case AST_STATE_VALUE:
                    value = child->value;
                    break;

                case AST_EXPRESSION:
                    value = child->value;
                    is_expression = 1;
                    break;

                case AST_DATA_TYPE:
                    data_type = child->value;
                    break;

                default:
                    break;
            }
        }

        if (name == NULL) {

            fprintf(
                stderr,
                "Runtime error: state has no name\n"
            );

            return RUNTIME_ERROR;
        }

        if (runtime_state_find(
                runtime->states,
                name) != NULL) {

            fprintf(
                stderr,
                "Runtime error: state '%s' "
                "is already registered\n",
                name
            );

            return RUNTIME_ERROR;
        }

        char evaluated_value[64];
        evaluated_value[0] = '\0';

        if (is_expression) {

            double result;

            if (!runtime_evaluate_expression(
                    runtime->states,
                    value,
                    &result)) {

                fprintf(
                    stderr,
                    "Runtime error: could not evaluate "
                    "expression for state '%s'\n",
                    name
                );

                return RUNTIME_ERROR;
            }

            snprintf(
                evaluated_value,
                sizeof(evaluated_value),
                "%.15g",
                result
            );

            value = evaluated_value;
        }

        RuntimeValueType type;

        if (data_type != NULL) {
            type = runtime_data_type(data_type);
        }
        else {
            type = runtime_scalar_type(value);
        }

        RuntimeState *runtime_state =
            runtime_state_create(
                name,
                type,
                value
            );

        if (runtime_state == NULL) {

            fprintf(
                stderr,
                "Runtime error: could not create "
                "state '%s'\n",
                name
            );

            return RUNTIME_ERROR;
        }

        if (data_type != NULL &&
            value != NULL) {

            if (!runtime_initialize_collection(
                    runtime_state,
                    value)) {

                fprintf(
                    stderr,
                    "Runtime error: could not initialize "
                    "state '%s'\n",
                    name
                );

                runtime_state_free(
                    runtime_state
                );

                return RUNTIME_ERROR;
            }
        }

        if (!runtime_state_store_add(
                runtime->states,
                runtime_state)) {

            fprintf(
                stderr,
                "Runtime error: could not register "
                "state '%s'\n",
                name
            );

            runtime_state_free(
                runtime_state
            );

            return RUNTIME_ERROR;
        }
    }

    return RUNTIME_SUCCESS;
}

static RuntimeResult runtime_execute_action(
    Runtime *runtime,
    RuntimeState *state,
    const ASTNode *action)
{
    if (runtime == NULL ||
        state == NULL ||
        action == NULL) {

        return RUNTIME_ERROR;
    }

    if (action->type == AST_PUSH) {

        if (action->child_count == 0) {

            fprintf(
                stderr,
                "Runtime error: push requires a value\n"
            );

            return RUNTIME_ERROR;
        }

        const ASTNode *value =
            action->children[0];

        if (value->value == NULL) {

            fprintf(
                stderr,
                "Runtime error: push value is empty\n"
            );

            return RUNTIME_ERROR;
        }

        if (state->value.type ==
            RUNTIME_VALUE_BRANCH) {

            if (!runtime_branch_insert(
                    state,
                    value->value)) {

                if (!runtime_branch_contains(
                        state,
                        value->value)) {

                    fprintf(
                        stderr,
                        "Runtime error: could not insert "
                        "into branch '%s'\n",
                        state->name
                    );

                    return RUNTIME_ERROR;
                }
            }

            return RUNTIME_SUCCESS;
        }

        if (!runtime_state_push(
                state,
                value->value)) {

            fprintf(
                stderr,
                "Runtime error: could not push "
                "into '%s'\n",
                state->name
            );

            return RUNTIME_ERROR;
        }

        return RUNTIME_SUCCESS;
    }

    if (action->type == AST_POP) {

        if (state->value.type ==
            RUNTIME_VALUE_BRANCH) {

            fprintf(
                stderr,
                "Runtime error: pop is not supported "
                "for branch '%s'\n",
                state->name
            );

            return RUNTIME_ERROR;
        }

        char *value =
            runtime_state_pop(state);

        if (value == NULL) {

            fprintf(
                stderr,
                "Runtime error: cannot pop empty "
                "state '%s'\n",
                state->name
            );

            return RUNTIME_ERROR;
        }

        printf(
            "POP %s: %s\n",
            state->name,
            value
        );

        free(value);

        return RUNTIME_SUCCESS;
    }

    fprintf(
        stderr,
        "Runtime error: unsupported data "
        "structure action\n"
    );

    return RUNTIME_ERROR;
}

RuntimeResult runtime_execute_data_structure_operation(
    Runtime *runtime,
    const ASTNode *operation)
{
    if (runtime == NULL ||
        operation == NULL) {

        return RUNTIME_ERROR;
    }

    if (operation->type !=
        AST_DATA_STRUCTURE_OPERATION) {

        fprintf(
            stderr,
            "Runtime error: invalid data "
            "structure operation node\n"
        );

        return RUNTIME_ERROR;
    }

    if (operation->value == NULL) {

        fprintf(
            stderr,
            "Runtime error: data structure "
            "has no name\n"
        );

        return RUNTIME_ERROR;
    }

    RuntimeState *state =
        runtime_state_find(
            runtime->states,
            operation->value
        );

    if (state == NULL) {

        fprintf(
            stderr,
            "Runtime error: unknown data "
            "structure '%s'\n",
            operation->value
        );

        return RUNTIME_ERROR;
    }

    const char *type_name = NULL;

    for (size_t i = 0;
         i < operation->child_count;
         i++) {

        const ASTNode *child =
            operation->children[i];

        if (child->type ==
            AST_DATA_TYPE) {

            type_name = child->value;
            break;
        }
    }

    if (type_name == NULL) {

        fprintf(
            stderr,
            "Runtime error: data structure "
            "operation for '%s' has no type\n",
            state->name
        );

        return RUNTIME_ERROR;
    }

    RuntimeValueType expected_type =
        runtime_data_type(type_name);

    if (state->value.type !=
        expected_type) {

        fprintf(
            stderr,
            "Runtime error: type mismatch for "
            "state '%s': operation expects %s, "
            "state is %s\n",
            state->name,
            type_name,
            runtime_value_type_name(
                state->value.type
            )
        );

        return RUNTIME_ERROR;
    }

    for (size_t i = 0;
         i < operation->child_count;
         i++) {

        const ASTNode *child =
            operation->children[i];

        if (child->type ==
            AST_DATA_TYPE) {

            continue;
        }

        if (child->type != AST_PUSH &&
            child->type != AST_POP) {

            fprintf(
                stderr,
                "Runtime error: unsupported child "
                "in data structure operation\n"
            );

            return RUNTIME_ERROR;
        }

        RuntimeResult result =
            runtime_execute_action(
                runtime,
                state,
                child
            );

        if (result != RUNTIME_SUCCESS) {
            return result;
        }
    }

    return RUNTIME_SUCCESS;
}

RuntimeResult runtime_execute_constant(
    Runtime *runtime,
    const ASTNode *constant)
{
    (void)runtime;

    if (constant == NULL) {
        return RUNTIME_ERROR;
    }

    if (constant->child_count == 0) {

        fprintf(
            stderr,
            "Runtime error: constant has no value\n"
        );

        return RUNTIME_ERROR;
    }

    const ASTNode *value =
        constant->children[0];

    printf(
        "CONSTANT: %s\n",
        value->value != NULL
        ? value->value
        : ""
    );

    return RUNTIME_SUCCESS;
}

static int runtime_evaluate_condition(
    Runtime *runtime,
    const ASTNode *condition)
{
    if (runtime == NULL ||
        condition == NULL) {

        return -1;
    }

    if (condition->type ==
        AST_EXPRESSION) {

        double result;

        if (!runtime_evaluate_expression(
                runtime->states,
                condition->value,
                &result)) {

            return -1;
        }

        return result != 0.0;
    }

    if (condition->type ==
        AST_RESULT) {

        if (condition->value == NULL) {
            return 0;
        }

        if (strcmp(
                condition->value,
                "true"
            ) == 0) {

            return 1;
        }

        if (strcmp(
                condition->value,
                "false"
            ) == 0) {

            return 0;
        }

        RuntimeState *state =
            runtime_state_find(
                runtime->states,
                condition->value
            );

        if (state == NULL) {

            fprintf(
                stderr,
                "Runtime error: unknown state "
                "'%s' in condition\n",
                condition->value
            );

            return -1;
        }

        return runtime_value_is_true(
            &state->value
        );
    }

    if (condition->type ==
        AST_STATE_NAME) {

        RuntimeState *state =
            runtime_state_find(
                runtime->states,
                condition->value
            );

        if (state == NULL) {

            fprintf(
                stderr,
                "Runtime error: unknown state "
                "'%s' in condition\n",
                condition->value
            );

            return -1;
        }

        return runtime_value_is_true(
            &state->value
        );
    }

    if (condition->value != NULL) {

        double result;

        if (runtime_evaluate_expression(
                runtime->states,
                condition->value,
                &result)) {

            return result != 0.0;
        }
    }

    fprintf(
        stderr,
        "Runtime error: unsupported condition\n"
    );

    return -1;
}

static RuntimeResult runtime_execute_node(
    Runtime *runtime,
    const ASTNode *node);

static RuntimeResult runtime_execute_children(
    Runtime *runtime,
    const ASTNode *node,
    size_t start)
{
    if (runtime == NULL ||
        node == NULL) {

        return RUNTIME_ERROR;
    }

    for (size_t i = start;
         i < node->child_count;
         i++) {

        RuntimeResult result =
            runtime_execute_node(
                runtime,
                node->children[i]
            );

        if (result != RUNTIME_SUCCESS) {
            return result;
        }
    }

    return RUNTIME_SUCCESS;
}

static RuntimeResult runtime_execute_if_branch(
    Runtime *runtime,
    const ASTNode *branch)
{
    if (runtime == NULL ||
        branch == NULL) {

        return RUNTIME_ERROR;
    }

    if (branch->child_count == 0) {

        fprintf(
            stderr,
            "Runtime error: conditional branch "
            "has no condition\n"
        );

        return RUNTIME_ERROR;
    }

    int condition =
        runtime_evaluate_condition(
            runtime,
            branch->children[0]
        );

    if (condition < 0) {
        return RUNTIME_ERROR;
    }

    if (!condition) {
        return RUNTIME_SUCCESS;
    }

    return runtime_execute_children(
        runtime,
        branch,
        1
    );
}

static RuntimeResult runtime_execute_else_branch(
    Runtime *runtime,
    const ASTNode *branch)
{
    if (runtime == NULL ||
        branch == NULL) {

        return RUNTIME_ERROR;
    }

    return runtime_execute_children(
        runtime,
        branch,
        0
    );
}

static RuntimeResult runtime_execute_if_chain(
    Runtime *runtime,
    const ASTNode *logic,
    size_t *index)
{
    if (runtime == NULL ||
        logic == NULL ||
        index == NULL) {

        return RUNTIME_ERROR;
    }

    size_t current = *index;

    const ASTNode *if_node =
        logic->children[current];

    RuntimeResult result =
        runtime_execute_if_branch(
            runtime,
            if_node
        );

    if (result != RUNTIME_SUCCESS) {

        int condition =
            runtime_evaluate_condition(
                runtime,
                if_node->children[0]
            );

        if (condition < 0) {
            return RUNTIME_ERROR;
        }

        return result;
    }

    int if_condition =
        runtime_evaluate_condition(
            runtime,
            if_node->children[0]
        );

    if (if_condition < 0) {
        return RUNTIME_ERROR;
    }

    if (if_condition) {

        current++;

        while (current <
               logic->child_count) {

            ASTType type =
                logic->children[current]->type;

            if (type != AST_ELSE_IF &&
                type != AST_ELSE) {

                break;
            }

            current++;
        }

        *index = current - 1;

        return RUNTIME_SUCCESS;
    }

    current++;

    while (current <
           logic->child_count) {

        const ASTNode *branch =
            logic->children[current];

        if (branch->type ==
            AST_ELSE_IF) {

            if (branch->child_count == 0) {

                fprintf(
                    stderr,
                    "Runtime error: else-if "
                    "has no condition\n"
                );

                return RUNTIME_ERROR;
            }

            int condition =
                runtime_evaluate_condition(
                    runtime,
                    branch->children[0]
                );

            if (condition < 0) {
                return RUNTIME_ERROR;
            }

            if (condition) {

                RuntimeResult execution =
                    runtime_execute_children(
                        runtime,
                        branch,
                        1
                    );

                if (execution !=
                    RUNTIME_SUCCESS) {

                    return execution;
                }

                current++;

                while (
                    current <
                    logic->child_count
                ) {

                    ASTType type =
                        logic->children[
                            current
                        ]->type;

                    if (type != AST_ELSE_IF &&
                        type != AST_ELSE) {

                        break;
                    }

                    current++;
                }

                *index = current - 1;

                return RUNTIME_SUCCESS;
            }

            current++;
            continue;
        }

        if (branch->type == AST_ELSE) {

            RuntimeResult execution =
                runtime_execute_else_branch(
                    runtime,
                    branch
                );

            if (execution !=
                RUNTIME_SUCCESS) {

                return execution;
            }

            *index = current;

            return RUNTIME_SUCCESS;
        }

        break;
    }

    *index = current - 1;

    return RUNTIME_SUCCESS;
}

RuntimeResult runtime_execute_logic(
    Runtime *runtime,
    const ASTNode *logic_node)
{
    if (runtime == NULL ||
        logic_node == NULL) {

        return RUNTIME_ERROR;
    }

    size_t i = 0;

    if (logic_node->child_count > 0 &&
        logic_node->children[0]->type ==
            AST_EXPRESSION) {

        i = 1;
    }

    while (i < logic_node->child_count) {

        const ASTNode *child =
            logic_node->children[i];

        if (child->type == AST_IF) {

            RuntimeResult result =
                runtime_execute_if_chain(
                    runtime,
                    logic_node,
                    &i
                );

            if (result !=
                RUNTIME_SUCCESS) {

                return result;
            }

            i++;
            continue;
        }

        if (child->type == AST_ELSE_IF ||
            child->type == AST_ELSE) {

            fprintf(
                stderr,
                "Runtime error: else/else-if "
                "without preceding if\n"
            );

            return RUNTIME_ERROR;
        }

        RuntimeResult result =
            runtime_execute_node(
                runtime,
                child
            );

        if (result !=
            RUNTIME_SUCCESS) {

            return result;
        }

        i++;
    }

    return RUNTIME_SUCCESS;
}

RuntimeResult runtime_execute_transition(
    Runtime *runtime,
    const ASTNode *transition)
{
    if (runtime == NULL ||
        transition == NULL) {

        return RUNTIME_ERROR;
    }

    const char *target =
        transition->value;

    if (target == NULL &&
        transition->child_count > 0) {

        for (size_t i = 0;
             i < transition->child_count;
             i++) {

            const ASTNode *child =
                transition->children[i];

            if (child->value != NULL) {
                target = child->value;
                break;
            }
        }
    }

    if (target == NULL ||
        target[0] == '\0') {

        fprintf(
            stderr,
            "Runtime error: transition has "
            "no target\n"
        );

        return RUNTIME_ERROR;
    }

    if (runtime_state_find(
            runtime->states,
            target) == NULL) {

        fprintf(
            stderr,
            "Runtime error: invalid transition "
            "target '%s'\n",
            target
        );

        return RUNTIME_ERROR;
    }

    char *new_state =
        copy_string(target);

    if (new_state == NULL) {
        return RUNTIME_ERROR;
    }

    free(runtime->current_state);

    runtime->current_state =
        new_state;

    printf(
        "TRANSITION: %s\n",
        runtime->current_state
    );

    return RUNTIME_SUCCESS;
}

RuntimeResult runtime_execute_context(
    Runtime *runtime,
    const ASTNode *context)
{
    if (runtime == NULL ||
        context == NULL) {

        return RUNTIME_ERROR;
    }

    if (context->child_count == 0) {

        fprintf(
            stderr,
            "Runtime error: context has no condition\n"
        );

        return RUNTIME_ERROR;
    }

    int condition =
        runtime_evaluate_condition(
            runtime,
            context->children[0]
        );

    if (condition < 0) {
        return RUNTIME_ERROR;
    }

    if (!condition) {
        return RUNTIME_SUCCESS;
    }

    return runtime_execute_children(
        runtime,
        context,
        1
    );
}

RuntimeResult runtime_execute_rule(
    Runtime *runtime,
    const ASTNode *rule)
{
    if (runtime == NULL ||
        rule == NULL) {

        return RUNTIME_ERROR;
    }

    if (rule->child_count == 0) {

        fprintf(
            stderr,
            "Runtime error: rule has no condition\n"
        );

        return RUNTIME_ERROR;
    }

    int condition =
        runtime_evaluate_condition(
            runtime,
            rule->children[0]
        );

    if (condition < 0) {
        return RUNTIME_ERROR;
    }

    if (!condition) {
        return RUNTIME_SUCCESS;
    }

    return runtime_execute_children(
        runtime,
        rule,
        1
    );
}

RuntimeResult runtime_execute_contract(
    Runtime *runtime,
    const ASTNode *contract)
{
    if (runtime == NULL ||
        contract == NULL) {

        return RUNTIME_ERROR;
    }

    printf(
        "CONTRACT: %s\n",
        contract->value != NULL
        ? contract->value
        : ""
    );

    for (size_t i = 0;
         i < contract->child_count;
         i++) {

        const ASTNode *child =
            contract->children[i];

        RuntimeResult result =
            runtime_execute_node(
                runtime,
                child
            );

        if (result !=
            RUNTIME_SUCCESS) {

            return result;
        }
    }

    return RUNTIME_SUCCESS;
}

RuntimeResult runtime_execute_result(
    Runtime *runtime,
    const ASTNode *result)
{
    if (runtime == NULL ||
        result == NULL) {

        return RUNTIME_ERROR;
    }

    const char *value =
        result->value;

    if (value == NULL &&
        result->child_count > 0) {

        const ASTNode *child =
            result->children[0];

        value = child->value;
    }

    if (value == NULL) {

        fprintf(
            stderr,
            "Runtime error: result has no value\n"
        );

        return RUNTIME_ERROR;
    }

    if (strcmp(value, "true") == 0 ||
        strcmp(value, "success") == 0 ||
        strcmp(value, "1") == 0) {

        printf("RESULT: success\n");
        return RUNTIME_SUCCESS;
    }

    if (strcmp(value, "false") == 0 ||
        strcmp(value, "failure") == 0 ||
        strcmp(value, "0") == 0) {

        printf("RESULT: failure\n");
        return RUNTIME_ERROR;
    }

    RuntimeState *state =
        runtime_state_find(
            runtime->states,
            value
        );

    if (state != NULL) {

        if (runtime_value_is_true(
                &state->value)) {

            printf("RESULT: success\n");
            return RUNTIME_SUCCESS;
        }

        printf("RESULT: failure\n");
        return RUNTIME_ERROR;
    }

    fprintf(
        stderr,
        "Runtime error: unknown result '%s'\n",
        value
    );

    return RUNTIME_ERROR;
}

RuntimeResult runtime_execute_execute(
    Runtime *runtime,
    const ASTNode *execute_node)
{
    if (runtime == NULL ||
        execute_node == NULL) {

        return RUNTIME_ERROR;
    }

    printf("EXECUTE\n");

    return runtime_execute_children(
        runtime,
        execute_node,
        0
    );
}

RuntimeResult runtime_execute_terminate(
    Runtime *runtime,
    const ASTNode *terminate)
{
    (void)runtime;
    (void)terminate;

    printf("TERMINATE\n");

    return RUNTIME_TERMINATE;
}

static RuntimeResult runtime_execute_node(
    Runtime *runtime,
    const ASTNode *node)
{
    if (runtime == NULL ||
        node == NULL) {

        return RUNTIME_ERROR;
    }

    switch (node->type) {

        case AST_REGISTER:
            return runtime_execute_register(
                runtime,
                node
            );

        case AST_LOGIC:
            return runtime_execute_logic(
                runtime,
                node
            );

        case AST_CONSTANT:
            return runtime_execute_constant(
                runtime,
                node
            );

        case AST_DATA_STRUCTURE_OPERATION:
            return runtime_execute_data_structure_operation(
                runtime,
                node
            );

        case AST_IF:
            return runtime_execute_if_branch(
                runtime,
                node
            );

        case AST_ELSE_IF:
            return runtime_execute_if_branch(
                runtime,
                node
            );

        case AST_ELSE:
            return runtime_execute_else_branch(
                runtime,
                node
            );

        case AST_TRANSITION:
            return runtime_execute_transition(
                runtime,
                node
            );

        case AST_CONTEXT:
            return runtime_execute_context(
                runtime,
                node
            );

        case AST_RULE:
            return runtime_execute_rule(
                runtime,
                node
            );

        case AST_CONTRACT_CALL:
            return runtime_execute_contract(
                runtime,
                node
            );

        case AST_RESULT:
            return runtime_execute_result(
                runtime,
                node
            );

        case AST_EXECUTE:
            return runtime_execute_execute(
                runtime,
                node
            );

        case AST_TERMINATE:
            return runtime_execute_terminate(
                runtime,
                node
            );

        case AST_PROGRAM:
        case AST_STATE_DECLARATION:
        case AST_STATE_NAME:
        case AST_STATE_VALUE:
        case AST_DATA_TYPE:
        case AST_DATA_ITEMS:
        case AST_EXPRESSION:
        case AST_PUSH:
        case AST_POP:
        case AST_LIST:
        case AST_QUEUE:
        case AST_STACK:
        case AST_BRANCH:

            fprintf(
                stderr,
                "Runtime error: non-executable AST "
                "node '%s' reached runtime\n",
                ast_type_name(node->type)
            );

            return RUNTIME_ERROR;

        default:

            fprintf(
                stderr,
                "Runtime error: unsupported AST node\n"
            );

            return RUNTIME_ERROR;
    }
}

RuntimeResult runtime_execute(
    Runtime *runtime,
    const ASTNode *program)
{
    if (runtime == NULL ||
        program == NULL) {

        return RUNTIME_ERROR;
    }

    if (program->type != AST_PROGRAM) {

        fprintf(
            stderr,
            "Runtime error: root AST node "
            "must be PROGRAM\n"
        );

        return RUNTIME_ERROR;
    }

    for (size_t i = 0;
         i < program->child_count;
         i++) {

        RuntimeResult result =
            runtime_execute_node(
                runtime,
                program->children[i]
            );

        if (result == RUNTIME_TERMINATE) {
            return RUNTIME_TERMINATE;
        }

        if (result == RUNTIME_ERROR) {
            return RUNTIME_ERROR;
        }
    }

    return RUNTIME_SUCCESS;
}

void runtime_print_state(
    const Runtime *runtime)
{
    if (runtime == NULL) {
        return;
    }

    runtime_state_print(
        runtime->states
    );

    if (runtime->current_state != NULL) {

        printf(
            "CURRENT STATE: %s\n",
            runtime->current_state
        );
    }
}
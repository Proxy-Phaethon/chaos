#include "runtime.h"

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>

typedef struct {
const char *input;
size_t position;
RuntimeStateStore *states;
} ExpressionParser;

static int runtime_evaluate_expression(
RuntimeStateStore *states,
const char *expression,
double *result
);

static RuntimeValueType runtime_data_type(
const char *type)
{
if (type == NULL) {
return RUNTIME_VALUE_STRING;
}

```
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
```

}

static RuntimeValueType runtime_scalar_type(
const char *value)
{
if (value == NULL) {
return RUNTIME_VALUE_STRING;
}

```
char *end = NULL;

strtod(value, &end);

if (end != value && *end == '\0') {
    return RUNTIME_VALUE_NUMBER;
}

return RUNTIME_VALUE_STRING;
```

}

Runtime *runtime_create(void)
{
Runtime *runtime = calloc(
1,
sizeof(Runtime)
);

```
if (runtime == NULL) {
    return NULL;
}

runtime->states =
    runtime_state_store_create();

if (runtime->states == NULL) {
    free(runtime);
    return NULL;
}

return runtime;
```

}

void runtime_free(
Runtime *runtime)
{
if (runtime == NULL) {
return;
}

```
runtime_state_store_free(
    runtime->states
);

free(runtime);
```

}

static int runtime_initialize_collection(
RuntimeState *state,
const char *value)
{
if (state == NULL ||
value == NULL ||
value[0] == '\0') {

```
    return 1;
}

char *contents = malloc(
    strlen(value) + 1
);

if (contents == NULL) {
    return 0;
}

strcpy(contents, value);

char *item = strtok(
    contents,
    ","
);

while (item != NULL) {

    while (*item == ' ') {
        item++;
    }

    size_t length = strlen(item);

    while (length > 0 &&
           (item[length - 1] == ' ' ||
            item[length - 1] == '\n' ||
            item[length - 1] == '\r')) {

        item[length - 1] = '\0';
        length--;
    }

    length = strlen(item);

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
```

}

RuntimeResult runtime_execute_register(
Runtime *runtime,
const ASTNode *register_node)
{
if (runtime == NULL ||
register_node == NULL) {

```
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

    char evaluated_value[64];

    if (is_expression) {

        double result;

        if (!runtime_evaluate_expression(
                runtime->states,
                value,
                &result)) {

            fprintf(
                stderr,
                "Runtime error: could not evaluate expression for state '%s'\n",
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
            "Runtime error: could not create state '%s'\n",
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
                "Runtime error: could not initialize state '%s'\n",
                name
            );

            free(runtime_state->name);

            runtime_value_free_contents(
                &runtime_state->value
            );

            free(runtime_state);

            return RUNTIME_ERROR;
        }
    }

    runtime_state_store_add(
        runtime->states,
        runtime_state
    );
}

return RUNTIME_SUCCESS;
```

}

static RuntimeResult runtime_execute_action(
Runtime *runtime,
RuntimeState *state,
const ASTNode *action)
{
if (runtime == NULL ||
state == NULL ||
action == NULL) {

```
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

    if (!runtime_state_push(
            state,
            value->value)) {

        fprintf(
            stderr,
            "Runtime error: could not push into '%s'\n",
            state->name
        );

        return RUNTIME_ERROR;
    }

    return RUNTIME_SUCCESS;
}

if (action->type == AST_POP) {

    char *value =
        runtime_state_pop(state);

    if (value == NULL) {
        fprintf(
            stderr,
            "Runtime error: cannot pop empty state '%s'\n",
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

return RUNTIME_ERROR;
```

}

RuntimeResult runtime_execute_data_structure_operation(
Runtime *runtime,
const ASTNode *operation)
{
if (runtime == NULL ||
operation == NULL) {

```
    return RUNTIME_ERROR;
}

if (operation->type !=
    AST_DATA_STRUCTURE_OPERATION) {

    return RUNTIME_ERROR;
}

if (operation->value == NULL) {
    fprintf(
        stderr,
        "Runtime error: data structure has no name\n"
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
        "Runtime error: unknown data structure '%s'\n",
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

    if (child->type == AST_DATA_TYPE) {
        type_name = child->value;
        break;
    }
}

if (type_name == NULL) {
    fprintf(
        stderr,
        "Runtime error: data structure operation for '%s' has no type\n",
        state->name
    );

    return RUNTIME_ERROR;
}

RuntimeValueType expected_type =
    runtime_data_type(type_name);

if (state->value.type != expected_type) {
    fprintf(
        stderr,
        "Runtime error: type mismatch for state '%s': operation expects %s, state is %s\n",
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

    if (child->type == AST_DATA_TYPE) {
        continue;
    }

    if (child->type != AST_PUSH &&
        child->type != AST_POP) {

        continue;
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
```

}

RuntimeResult runtime_execute_constant(
Runtime *runtime,
const ASTNode *constant)
{
(void)runtime;

```
if (constant == NULL) {
    return RUNTIME_ERROR;
}

if (constant->child_count == 0) {
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
```

}

static RuntimeResult runtime_execute_operation(
Runtime *runtime,
const ASTNode *operation)
{
if (runtime == NULL ||
operation == NULL) {

```
    return RUNTIME_ERROR;
}

switch (operation->type) {

    case AST_TERMINATE:
        printf("TERMINATE\n");
        return RUNTIME_TERMINATE;

    case AST_DATA_STRUCTURE_OPERATION:
        return runtime_execute_data_structure_operation(
            runtime,
            operation
        );

    case AST_CONSTANT:
        return runtime_execute_constant(
            runtime,
            operation
        );

    case AST_IF:
    case AST_ELSE_IF:
    case AST_ELSE:
        /*
         * Conditional nodes are handled by
         * runtime_execute_conditional().
         */
        break;

    default:
        break;
}

return RUNTIME_ERROR;
```

}

static int runtime_condition_is_true(
Runtime *runtime,
const ASTNode *condition,
int *result)
{
if (runtime == NULL ||
condition == NULL ||
result == NULL) {

```
    return 0;
}

if (condition->type == AST_RESULT) {
    fprintf(
        stderr,
        "Runtime error: result conditions are not implemented\n"
    );

    return 0;
}

if (condition->value == NULL) {
    fprintf(
        stderr,
        "Runtime error: condition is empty\n"
    );

    return 0;
}

double value;

if (!runtime_evaluate_expression(
        runtime->states,
        condition->value,
        &value)) {

    fprintf(
        stderr,
        "Runtime error: could not evaluate condition\n"
    );

    return 0;
}

*result = value != 0.0;

return 1;
```

}

static RuntimeResult runtime_execute_node(
Runtime *runtime,
const ASTNode *node);

static RuntimeResult runtime_execute_conditional(
Runtime *runtime,
const ASTNode *node)
{
if (runtime == NULL ||
node == NULL) {

```
    return RUNTIME_ERROR;
}

if (node->child_count < 2) {
    fprintf(
        stderr,
        "Runtime error: conditional requires a condition and operation\n"
    );

    return RUNTIME_ERROR;
}

const ASTNode *condition =
    node->children[0];

int condition_true = 0;

if (!runtime_condition_is_true(
        runtime,
        condition,
        &condition_true)) {

    return RUNTIME_ERROR;
}

if (!condition_true) {
    return RUNTIME_SUCCESS;
}

for (size_t i = 1;
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
```

}

static RuntimeResult runtime_execute_if_chain(
Runtime *runtime,
const ASTNode *logic,
size_t *index)
{
if (runtime == NULL ||
logic == NULL ||
index == NULL) {

```
    return RUNTIME_ERROR;
}

const ASTNode *if_node =
    logic->children[*index];

if (if_node->type != AST_IF) {
    return RUNTIME_ERROR;
}

int branch_selected = 0;

if (if_node->child_count < 2) {
    fprintf(
        stderr,
        "Runtime error: if requires a condition and operation\n"
    );

    return RUNTIME_ERROR;
}

int condition_true = 0;

if (!runtime_condition_is_true(
        runtime,
        if_node->children[0],
        &condition_true)) {

    return RUNTIME_ERROR;
}

if (condition_true) {

    branch_selected = 1;

    for (size_t j = 1;
         j < if_node->child_count;
         j++) {

        RuntimeResult result =
            runtime_execute_node(
                runtime,
                if_node->children[j]
            );

        if (result != RUNTIME_SUCCESS) {
            return result;
        }
    }
}

size_t next = *index + 1;

while (next < logic->child_count) {

    const ASTNode *branch =
        logic->children[next];

    if (branch->type != AST_ELSE_IF &&
        branch->type != AST_ELSE) {

        break;
    }

    if (branch_selected) {
        next++;
        continue;
    }

    if (branch->type == AST_ELSE_IF) {

        if (branch->child_count < 2) {
            fprintf(
                stderr,
                "Runtime error: else if requires a condition and operation\n"
            );

            return RUNTIME_ERROR;
        }

        if (!runtime_condition_is_true(
                runtime,
                branch->children[0],
                &condition_true)) {

            return RUNTIME_ERROR;
        }

        if (!condition_true) {
            next++;
            continue;
        }

        branch_selected = 1;

        for (size_t j = 1;
             j < branch->child_count;
             j++) {

            RuntimeResult result =
                runtime_execute_node(
                    runtime,
                    branch->children[j]
                );

            if (result != RUNTIME_SUCCESS) {
                return result;
            }
        }

        next++;
        continue;
    }

    if (branch->type == AST_ELSE) {

        branch_selected = 1;

        for (size_t j = 0;
             j < branch->child_count;
             j++) {

            RuntimeResult result =
                runtime_execute_node(
                    runtime,
                    branch->children[j]
                );

            if (result != RUNTIME_SUCCESS) {
                return result;
            }
        }

        next++;
        break;
    }
}

*index = next - 1;

return RUNTIME_SUCCESS;
```

}

static RuntimeResult runtime_execute_node(
Runtime *runtime,
const ASTNode *node)
{
if (runtime == NULL ||
node == NULL) {

```
    return RUNTIME_ERROR;
}

switch (node->type) {

    case AST_IF:
    case AST_ELSE_IF:
    case AST_ELSE:
        return runtime_execute_conditional(
            runtime,
            node
        );

    case AST_TERMINATE:
        return runtime_execute_operation(
            runtime,
            node
        );

    case AST_DATA_STRUCTURE_OPERATION:
        return runtime_execute_data_structure_operation(
            runtime,
            node
        );

    case AST_CONSTANT:
        return runtime_execute_constant(
            runtime,
            node
        );

    default:
        return RUNTIME_ERROR;
}
```

}

RuntimeResult runtime_execute_logic(
Runtime *runtime,
const ASTNode *logic_node)
{
if (runtime == NULL ||
logic_node == NULL) {

```
    return RUNTIME_ERROR;
}

for (size_t i = 0;
     i < logic_node->child_count;
     i++) {

    const ASTNode *child =
        logic_node->children[i];

    if (child->type == AST_IF) {

        RuntimeResult result =
            runtime_execute_if_chain(
                runtime,
                logic_node,
                &i
            );

        if (result != RUNTIME_SUCCESS) {
            return result;
        }

        continue;
    }

    if (child->type == AST_ELSE_IF ||
        child->type == AST_ELSE) {

        fprintf(
            stderr,
            "Runtime error: 'else if' or 'else' without 'if'\n"
        );

        return RUNTIME_ERROR;
    }

    switch (child->type) {

        case AST_CONSTANT:
            {
                RuntimeResult result =
                    runtime_execute_constant(
                        runtime,
                        child
                    );

                if (result != RUNTIME_SUCCESS) {
                    return result;
                }
            }
            break;

        case AST_DATA_STRUCTURE_OPERATION:
            {
                RuntimeResult result =
                    runtime_execute_data_structure_operation(
                        runtime,
                        child
                    );

                if (result != RUNTIME_SUCCESS) {
                    return result;
                }
            }
            break;

        case AST_TERMINATE:
            return runtime_execute_operation(
                runtime,
                child
            );

        case AST_EXPRESSION:
            /*
             * The first child of logic is the
             * logic question. It is evaluated
             * only as part of the logic condition
             * itself and does not execute as an
             * operation.
             */
            break;

        case AST_TRANSITION:
        case AST_CONTEXT:
        case AST_RULE:
        case AST_CONTRACT_CALL:
        case AST_RESULT:
            /*
             * These remain parser/AST features.
             * Their runtime semantics are handled
             * in later phases.
             */
            break;

        default:
            break;
    }
}

return RUNTIME_SUCCESS;
```

}

RuntimeResult runtime_execute_execute(
Runtime *runtime,
const ASTNode *execute_node)
{
(void)runtime;
(void)execute_node;

```
printf("EXECUTE\n");

return RUNTIME_SUCCESS;
```

}

RuntimeResult runtime_execute(
Runtime *runtime,
const ASTNode *program)
{
if (runtime == NULL ||
program == NULL) {

```
    return RUNTIME_ERROR;
}

if (program->type != AST_PROGRAM) {
    return RUNTIME_ERROR;
}

for (size_t i = 0;
     i < program->child_count;
     i++) {

    const ASTNode *node =
        program->children[i];

    RuntimeResult result;

    switch (node->type) {

        case AST_REGISTER:
            result =
                runtime_execute_register(
                    runtime,
                    node
                );
            break;

        case AST_LOGIC:
            result =
                runtime_execute_logic(
                    runtime,
                    node
                );
            break;

        case AST_EXECUTE:
            result =
                runtime_execute_execute(
                    runtime,
                    node
                );
            break;

        default:
            continue;
    }

    if (result == RUNTIME_TERMINATE) {
        return RUNTIME_TERMINATE;
    }

    if (result == RUNTIME_ERROR) {
        return RUNTIME_ERROR;
    }
}

return RUNTIME_SUCCESS;
```

}

void runtime_print_state(
const Runtime *runtime)
{
if (runtime == NULL) {
return;
}

```
runtime_state_print(
    runtime->states
);
```

}

static void expression_skip_spaces(
ExpressionParser *parser)
{
while (isspace(
(unsigned char)parser->input[
parser->position
])) {

```
    parser->position++;
}
```

}

static char expression_current(
ExpressionParser *parser)
{
expression_skip_spaces(parser);

```
return parser->input[
    parser->position
];
```

}

static int expression_parse_comparison(
ExpressionParser *parser,
double *result);

static int expression_parse_factor(
ExpressionParser *parser,
double *result)
{
expression_skip_spaces(parser);

```
char current =
    expression_current(parser);

if (current == '+') {

    parser->position++;

    return expression_parse_factor(
        parser,
        result
    );
}

if (current == '-') {

    parser->position++;

    if (!expression_parse_factor(
            parser,
            result)) {

        return 0;
    }

    *result = -*result;

    return 1;
}

if (current == '(') {

    parser->position++;

    if (!expression_parse_comparison(
            parser,
            result)) {

        return 0;
    }

    if (expression_current(parser) != ')') {

        fprintf(
            stderr,
            "Runtime error: expected ')' in expression\n"
        );

        return 0;
    }

    parser->position++;

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

    RuntimeState *state =
        runtime_state_find(
            parser->states,
            name
        );

    if (state == NULL) {

        fprintf(
            stderr,
            "Runtime error: unknown state '%s' in expression\n",
            name
        );

        return 0;
    }

    if (state->value.type !=
        RUNTIME_VALUE_NUMBER) {

        fprintf(
            stderr,
            "Runtime error: state '%s' is not a number\n",
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
            "Runtime error: state '%s' contains an invalid number\n",
            name
        );

        return 0;
    }

    *result = value;

    return 1;
}

fprintf(
    stderr,
    "Runtime error: unexpected character '%c' in expression\n",
    current
);

return 0;
```

}

static int expression_parse_term(
ExpressionParser *parser,
double *result)
{
if (!expression_parse_factor(
parser,
result)) {

```
    return 0;
}

while (1) {

    char operator =
        expression_current(parser);

    if (operator != '*' &&
        operator != '/') {

        break;
    }

    parser->position++;

    double right;

    if (!expression_parse_factor(
            parser,
            &right)) {

        return 0;
    }

    if (operator == '*') {
        *result *= right;
    }
    else {

        if (right == 0.0) {

            fprintf(
                stderr,
                "Runtime error: division by zero\n"
            );

            return 0;
        }

        *result /= right;
    }
}

return 1;
```

}

static int expression_parse_arithmetic(
ExpressionParser *parser,
double *result)
{
if (!expression_parse_term(
parser,
result)) {

```
    return 0;
}

while (1) {

    char operator =
        expression_current(parser);

    if (operator != '+' &&
        operator != '-') {

        break;
    }

    parser->position++;

    double right;

    if (!expression_parse_term(
            parser,
            &right)) {

        return 0;
    }

    if (operator == '+') {
        *result += right;
    }
    else {
        *result -= right;
    }
}

return 1;
```

}

static int expression_parse_comparison(
ExpressionParser *parser,
double *result)
{
if (!expression_parse_arithmetic(
parser,
result)) {

```
    return 0;
}

while (1) {

    expression_skip_spaces(parser);

    const char *input =
        parser->input +
        parser->position;

    if (strncmp(input, "<=", 2) == 0 ||
        strncmp(input, ">=", 2) == 0 ||
        strncmp(input, "==", 2) == 0 ||
        strncmp(input, "!=", 2) == 0) {

        double left = *result;
        double right;

        char first = input[0];
        char second = input[1];

        parser->position += 2;

        if (!expression_parse_arithmetic(
                parser,
                &right)) {

            return 0;
        }

        if (first == '<' && second == '=') {
            *result = left <= right;
        }
        else if (first == '>' && second == '=') {
            *result = left >= right;
        }
        else if (first == '=' && second == '=') {
            *result = left == right;
        }
        else {
            *result = left != right;
        }

        continue;
    }

    char operator =
        expression_current(parser);

    if (operator != '<' &&
        operator != '>') {

        break;
    }

    parser->position++;

    double left = *result;
    double right;

    if (!expression_parse_arithmetic(
            parser,
            &right)) {

        return 0;
    }

    if (operator == '<') {
        *result = left < right;
    }
    else {
        *result = left > right;
    }
}

return 1;
```

}

static int runtime_evaluate_expression(
RuntimeStateStore *states,
const char *expression,
double *result)
{
if (states == NULL ||
expression == NULL ||
result == NULL) {

```
    return 0;
}

ExpressionParser parser = {
    .input = expression,
    .position = 0,
    .states = states
};

if (!expression_parse_comparison(
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
        "Runtime error: unexpected text in expression near '%s'\n",
        parser.input +
            parser.position
    );

    return 0;
}

return 1;
```

}

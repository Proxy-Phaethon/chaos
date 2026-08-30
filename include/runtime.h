#ifndef CHAOS_RUNTIME_H
#define CHAOS_RUNTIME_H

#include "ast.h"
#include "runtime_state.h"

typedef enum {
    RUNTIME_SUCCESS,
    RUNTIME_ERROR,
    RUNTIME_TERMINATE
} RuntimeResult;

typedef struct {
    RuntimeStateStore *states;
    char *current_state;
} Runtime;

Runtime *runtime_create(void);

void runtime_free(
    Runtime *runtime
);

RuntimeResult runtime_execute(
    Runtime *runtime,
    const ASTNode *program
);

RuntimeResult runtime_execute_register(
    Runtime *runtime,
    const ASTNode *register_node
);

RuntimeResult runtime_execute_logic(
    Runtime *runtime,
    const ASTNode *logic_node
);

RuntimeResult runtime_execute_data_structure_operation(
    Runtime *runtime,
    const ASTNode *operation
);

RuntimeResult runtime_execute_constant(
    Runtime *runtime,
    const ASTNode *constant
);

RuntimeResult runtime_execute_transition(
    Runtime *runtime,
    const ASTNode *transition
);

RuntimeResult runtime_execute_context(
    Runtime *runtime,
    const ASTNode *context
);

RuntimeResult runtime_execute_rule(
    Runtime *runtime,
    const ASTNode *rule
);

RuntimeResult runtime_execute_contract(
    Runtime *runtime,
    const ASTNode *contract
);

RuntimeResult runtime_execute_result(
    Runtime *runtime,
    const ASTNode *result
);

RuntimeResult runtime_execute_execute(
    Runtime *runtime,
    const ASTNode *execute_node
);

RuntimeResult runtime_execute_terminate(
    Runtime *runtime,
    const ASTNode *terminate
);

void runtime_print_state(
    const Runtime *runtime
);

#endif
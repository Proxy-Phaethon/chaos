#ifndef CHAOS_RUNTIME_H
#define CHAOS_RUNTIME_H

#include "ast.h"
#include "runtime_state.h"

typedef struct {
    RuntimeStateStore *states;
} Runtime;

Runtime *runtime_create(void);

void runtime_free(
    Runtime *runtime
);

int runtime_execute(
    Runtime *runtime,
    const ASTNode *program
);

int runtime_execute_register(
    Runtime *runtime,
    const ASTNode *register_node
);

int runtime_execute_logic(
    Runtime *runtime,
    const ASTNode *logic_node
);

int runtime_execute_data_structure_operation(
    Runtime *runtime,
    const ASTNode *operation
);

int runtime_execute_constant(
    Runtime *runtime,
    const ASTNode *constant
);

int runtime_execute_execute(
    Runtime *runtime,
    const ASTNode *execute_node
);

void runtime_print_state(
    const Runtime *runtime
);

#endif
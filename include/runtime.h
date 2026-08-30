#ifndef CHAOS_RUNTIME_H
#define CHAOS_RUNTIME_H

#include "ast.h"
#include "runtime_state.h"

typedef enum {
    RUNTIME_FLOW_CONTINUE,
    RUNTIME_FLOW_TERMINATE
} RuntimeFlow;

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

int runtime_evaluate_condition(
    Runtime *runtime,
    const ASTNode *condition
);

int runtime_execute_if(
    Runtime *runtime,
    const ASTNode *if_node
);

int runtime_execute_else_if(
    Runtime *runtime,
    const ASTNode *else_if_node
);

int runtime_execute_else(
    Runtime *runtime,
    const ASTNode *else_node
);

int runtime_execute_terminate(
    Runtime *runtime,
    const ASTNode *terminate_node
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
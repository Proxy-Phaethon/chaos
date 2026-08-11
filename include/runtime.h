#ifndef RUNTIME_H
#define RUNTIME_H

#include "parser.h"

void run_program(Statement *statements, int count, char *value, int size);
void run_statement(Statement *statement, char *value, int size);
int condition_matches(const char *value, const char *condition);

#endif
#ifndef PARSER_H
#define PARSER_H

int parse_chaos(const char *line, char *question, int size);
int parse_call(const char *line, char builtins[][64], int max_builtins);

#endif
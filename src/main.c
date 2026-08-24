#include "ast.h"
#include "lexer.h"
#include "parser.h"
#include "runtime.h"

#include <stdio.h>
#include <stdlib.h>

static char *read_file(const char *path)
{
    FILE *file = fopen(path, "rb");

    if (file == NULL) {
        perror(path);
        return NULL;
    }

    fseek(file, 0, SEEK_END);

    long size = ftell(file);

    if (size < 0) {
        fclose(file);
        return NULL;
    }

    rewind(file);

    char *buffer = malloc((size_t)size + 1);

    if (buffer == NULL) {
        fclose(file);
        return NULL;
    }

    size_t bytes_read =
        fread(buffer, 1, (size_t)size, file);

    buffer[bytes_read] = '\0';

    fclose(file);

    return buffer;
}

int main(int argc, char **argv)
{
    if (argc != 2) {
        fprintf(
            stderr,
            "Usage: %s <file.chaos>\n",
            argv[0]
        );

        return 1;
    }

    char *source = read_file(argv[1]);

    if (source == NULL) {
        return 1;
    }

    TokenList *tokens =
        lexer_tokenize(source);

    if (tokens == NULL) {
        free(source);
        return 1;
    }

    ASTNode *program =
        parser_parse(tokens);

    if (program == NULL) {
        lexer_free(tokens);
        free(source);
        return 1;
    }

    printf("Parsed successfully.\n\n");

    ast_print(program, 0);

    Runtime *runtime =
        runtime_create();

    if (runtime == NULL) {
        fprintf(
            stderr,
            "Runtime error: could not create runtime.\n"
        );

        ast_free(program);
        lexer_free(tokens);
        free(source);

        return 1;
    }

    printf("\nRuntime:\n");

    if (!runtime_execute(runtime, program)) {
        fprintf(
            stderr,
            "Runtime error: execution failed.\n"
        );

        runtime_free(runtime);
        ast_free(program);
        lexer_free(tokens);
        free(source);

        return 1;
    }

    printf("\nState store:\n");

    runtime_print_state(runtime);

    runtime_free(runtime);
    ast_free(program);
    lexer_free(tokens);
    free(source);

    return 0;
}
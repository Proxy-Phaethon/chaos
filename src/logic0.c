#include <stdio.h>

void logic0(const char *question)
{
    char answer[256];

    printf("%s\n", question);

    fgets(answer, sizeof(answer), stdin);
}
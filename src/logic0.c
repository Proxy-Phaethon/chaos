#include <stdio.h>

static char answer[256];

void logic0(const char *question)
{
    printf("%s\n", question);

    fgets(answer, sizeof(answer), stdin);
}

const char *logic0_value(void)
{
    return answer;
}
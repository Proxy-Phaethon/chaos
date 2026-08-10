#include <stdio.h>

int check_answer(int answer)
{
    if (answer == 1)
    {
        return 1;
    }
    else
    {
        return 0;
    }
}

int main(void)
{
    int answer;

    printf("Enter a number: ");
    scanf("%d", &answer);

    int result = check_answer(answer);

    if (result == 1)
    {
        printf("yes\n");
    }
    else
    {
        printf("no\n");
    }

    return 0;
}
#include <stddef.h>
#include "contracts.h"

int main(void)
{
    ContractFunction function;

    function = find_contract("double");

    if (function != NULL)
    {
        function("7");
    }

    return 0;
}
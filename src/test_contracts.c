#include "contracts.h"
#include <stddef.h>

int main(void)
{
    ContractFunction function;

    function = find_contract("print");

    if (function != NULL)
    {
        function("Hello from a contract.");
    }

    return 0;
}
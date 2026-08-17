## what is a register?
it's the simplest form of data storage i could think of. for the version one, glorified calculator, you only need immediate data values, maybe update it, etc., so i thought, why not just give it a name and keep it at the top of the file?

so that's a register, and it contains states with names, values, and a type of data structure if you want it to be one. 

register;
    state: x = 3,

    state: list_one, queue = {'item 1', 'item 2'};

and then you can just type state: list_one and then give it a task to do, like push/pop to add or remove items (based on the data structure), transition rules, or simply provide a value for an operation.
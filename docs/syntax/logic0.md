## how logic0 works
this is the most primitive function of chaos. it is, essentially, a conditional statement, but with additional perks (i think).

a logic0 statement can ask a question and receive an answer, then proceed as per the options it was given. 

so you can have it ask, what is your name? it'll get the answer, read it, and proceed with the options you gave it - if word, proceed, else if string('christopher'), ban, else terminate.

simple as that.

but the underlying workings have me scratching my head because, of course, i am not fluent in C. that is not a weakness - i am not fluent in my own mother tongue, either. such is life, and im only human after all.

for logic0 to work, it has several parts to it. a typical logic0 looks like this:

logic0 ('this is a question?')
call metastable, sanitizer
if word,
    action ('contract')
else if number,
    action ('contract')
else,
    terminate

that concludes a logic0. the text inside single quotes in the parenthesis signifies the text that will be printed as a question. logic0 doesn't really care what the question is - it only cares about the answers. the question is for humans to see and answer to.

the call line signals that the following words are 'built-ins', the first significant library of chaos. built-ins are basically little workers in C that you can call to do operational actions without having to type the code yourself. so 'sanitizer' makes sure the input is valid, not too long, not secretly a SQL injection, etc.

once logic0's question gets an input, it is stored as a temporary value. this value is sent to the called built-ins after they've been looked up in the built-in registry and confirmed to exist. the built-ins receive the value, do their thing, and send back the output.

however, before the output reaches logic0, it must go through a condition resolver.

see 'word' and 'number' in the if-else statements? those are conditions, the second library, and logic0 doesn't know anything aside from a condition. it cannot receive 'hallelujah' and know that this is a word. the condition resolver must do that, and send to logic0 the possible conditions that an input matches with.

once the logic0 receives the conditions, it's simple. it matches the conditions to the ones it has been given. if a condition matches, the corresponding action proceeds. if none of the given conditions match, the action is invalid and terminates.

that is it. 

now for an action to proceed, it relies on the third library, the contracts. this is like a smaller version of the built-ins. while the built-ins can do larger tasks like fully validate an input, a contract is only a line or two of code at best. it allows you to create a custom 'action' of your own, if any built-in doesn't satisfy you.

but, of course, if you are lazy, you can use a built-in instead of an action, like the last else statement using 'terminate'. 
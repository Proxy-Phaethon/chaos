## Working Notes as I build Chaos

this has perhaps grown way out of the original scope, which was to simply build a sort of universal code transpiler that could generate files in proper structure, translate simple words into code in target languages, and reduce the need to manage a backend and a frontend and a database on different sites, etc.

but the more i work on that, the more i find that other things piss me off more than having to log into GitHub with another app's authentication code. such as the syntax of Rust, or C. why??? why must it be this way? surely there is a more efficient way to develop syntax?

to find out the answer to this pressing question, i embarked on a journey to understand how programming languages are made. however, in today's world of AI-generated code and barely any humans offering help (rather choosing to mockyour workflow for not using Claude to write your code), it seems rather difficult to learn pure programming.

god bless the MIT OpenCourseWare lectures. they're from decades ago, and focus on technical aspects purely. no other source has taught me as much about programming that a few lecture notes from 'Computation Structures', of all things.

i'm not saying that people who generate code with AI are useless. in fact, i entirely support the use of AI for code generation alone. i sympathise with you folks - i hate typing out line after line of words and symbols i cannot read together or understand.

but i still want the experience of being a disney channel hacker. just open a terminal and start typign and boom, things come into being. no crying over AWS, not scrolling through reddit hate comments for a simple explanation, no having to fix the bugs made by Claude cause it decided to change my architecture into what it thinks is better in terms of industry standards.

im a student, not an employee. i dont NEED industry standards, i want what i built to run properly and then laugh with joy and the result. 

so, ahem, say hello to chaos. this wi;ll be my document to track my progress of building the chaos language and eventually the CLI for it, because i suck at keeping physical notes (god bless the times i stared at my own handwriting in utter devastation). at least here, the words are legible, even if nonsensical, rambling, or too philosophical at time.

Friday, August 7, 2026
Chaos has a few words in its vocabulary now, and they're all based on a computer's hardware. this makes it rather confusing for someone who KNOWS about hardware to use this language, cause similar words mean slightly different things, but hey, that's the fun part about languages. it has to confuse you for a bit with synonyms before enlightening you.

the most basic element of chaos is logic. logic0, to be specific. i'll explain in a bit. logic0, logic1, contracts, action, and sometimes mux + context, makes up a block. a block is something like a combinational device. it is a fully operational thing, a gear in a machine. the machine itself is a calculator (because it works like an actual calculator does). it can do operations, compute things, etc., but to make it BETTER than a normal calculator, you can give it a register. this is where it stores data, as states, in case you need to collect data and then use it elsewhere and then change it and store again and so on. 

hopefully i manage to sit down and write a formal document for the language, this is just my brain dump, my apologies to anyone who happens to read this.

if you connect a bunch of calculators together, it creates an engine. yes sort of surprising change of physical objects, but think of what an engine truly is. think of what it DOES, and imagine that. same way you imagine what a calculator does, not the literal flat device with buttons itself.

an engine is a functional app, at this stage. a folder in a normal project could be an engine, and its files, calculators. neat stack of blocks.

and you further connect all your engines together to create the brain, the file.chaos that is your entire software itself. voila. so simple.

now let's look at the basic blocks as of today:
1. logic0
2. logic1
3. contract
4. action
5. mux
6. context
7. register
8. state
9. transition
10. metastable
11. c.pipeline
12. e.pipeline
13. block
14. calculator
15. engine
16. brain
17. clock
18. anticlock
19. sequence
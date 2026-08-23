## Working Notes as I build Chaos

this has perhaps grown way out of the original scope, which was to simply build a sort of universal code transpiler that could generate files in proper structure, translate simple words into code in target languages, and reduce the need to manage a backend and a frontend and a database on different sites, etc.

but the more i work on that, the more i find that other things piss me off more than having to log into GitHub with another app's authentication code. such as the syntax of Rust, or C. why??? why must it be this way? surely there is a more efficient way to develop syntax?

to find out the answer to this pressing question, i embarked on a journey to understand how programming languages are made. however, in today's world of AI-generated code and barely any humans offering help (rather choosing to mockyour workflow for not using Claude to write your code), it seems rather difficult to learn pure programming.

god bless the MIT OpenCourseWare lectures. they're from decades ago, and focus on technical aspects purely. no other source has taught me as much about programming that a few lecture notes from 'Computation Structures', of all things.

i'm not saying that people who generate code with AI are useless. in fact, i entirely support the use of AI for code generation alone. i sympathise with you folks - i hate typing out line after line of words and symbols i cannot read together or understand.

but i still want the experience of being a disney channel hacker. just open a terminal and start typign and boom, things come into being. no crying over AWS, not scrolling through reddit hate comments for a simple explanation, no having to fix the bugs made by Claude cause it decided to change my architecture into what it thinks is better in terms of industry standards.

im a student, not an employee. i dont NEED industry standards, i want what i built to run properly and then laugh with joy and the result. 

so, ahem, say hello to chaos. this wi;ll be my document to track my progress of building the chaos language and eventually the CLI for it, because i suck at keeping physical notes (god bless the times i stared at my own handwriting in utter devastation). at least here, the words are legible, even if nonsensical, rambling, or too philosophical at time.

## Friday, August 7, 2026
(deleted)

## Sunday, August 9
(deleted)

## Monday, 10th August
today, the first bit of Chaos is being made (or at least i'm attempting to do so). logic0 is a primitive, and conceptually very simple. it asks a question, gets an input, sends that input to whatever little function was declared to do sanitization or verification, and then receives the new output, checks it against its allowed actions, and proceeds to the next step.

C, however, is terrible. goddammit.

## Tuesday, 11th August
the more i work on this the more i realize that it is lacking something vital. the world is stubborn in its rules, and one person alone cant change it. you need a movement, a revolution, a mass moving as one. 

i, unfortunately, am far too reserved to ever create nor join such a movement. i cant even talk to my childhood friend without staring at the sky in wonder at my deteriorating social skills.

that aside, reagrding chaos specifically, this made me realize one thing - as a project, sure, it might be useful on my portfolio. but as a tool? very unlikely. 

so chaos cannot be simply just a language. it cannot simply just remove the repetitions and boilerplates of programming and be successful. it needs to be useful to ME, as well as others who have the same aversion to syntax that i do.

in that i have gone full circle and landed back on making chaos a universal transpiler of sorts.

it will remain a programming language in structure, but get an additional layer through its dedicated CLI - a system that translates the pure chaos code into whatever target language is required by your hiring company. 

the world of capitalism may fall soon, but we still need to survive to see that, and a common person needs money, a job. including me.

## Sunday, 16th August
okay, alright, i seriously need to collect my thoughts.

first bit of chaos will be the parser. like genuinely, i hate building things step by step. i already know all the words i need, might as well build the parser in one go.

## Monday, 17th August
my face itches, i wonder if i am allergic to niacinamide. i sure hope not, or my dreams of becoming a model are over. i so crave the feeling of dressing up prettily and posing for photoshoots. it just seems incredibly satisfying, you know. catering to my own ego, so i can finally stop looking in a mirror and screaming but instead go 'fwaaaaaaa'.

i'm going off-track, ahem. today i'll attempt to add the register, state declarations, data structure types, constants, and push/pop commands to the lexer and parser.

## Wednesday, 19th August
i have to pause building the language now to understand more about how algorithms and data structures work under the hood. leetcode is messed up, why do they frame the questions so weirdly? but it's at least helping me figure out what kind of operations you need to be able to do with a mathematical environment.

12.26 p.m. - the lexer, parser, and AST are functional for V1 syntax. next milestone is the runtime. shit.

## Sunday, 23rd August
oops, skipped a few days. i forgot where i left off.

ah yes, the runtime.
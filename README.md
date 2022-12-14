
## What does a PC have?
### Adressable memory
The PC needs a place where it can store and read variables. The memory needs to have an address (in HEX).

### Executable coding language
The PC needs a language to write code in. The language needs to be executable by the PC. I can write my own language for it

### Input/Output
The PC needs a way to get input from the user and a way to output data to the user. The PC needs to be able to read from the keyboard and write to the screen.

### Memory for the code
The PC needs a place to store the code. The code needs to be stored in a way that the PC can read it.

## Concept
The PC is built as follows: It has registers for memory, which persist data (like a RAM). Those can be read and written to either with the CLI or by code.  
Then we also have the code itself, which has memory spots. They start later (probably with 0x100), because the memory comes first. Then we have a pointer (or an eye), which stares at a certain spot in the code memory. The eye steps one address down on user command and executes that line. Maybe later implement that the eye goes by itself instead of waiting for user input.
In the end, it more or less works like a touring machine. Code will hopefully be touring complete, but that isnt the focus in this repo.

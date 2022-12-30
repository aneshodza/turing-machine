## Welcome to my touring machine
This project is trying to be a touring machine. I wrote it in rust (this being my first rust project), to get to know the language a bit better. The code probably has a lot of potential for improvement and PRs are welcome.

## Concept
In this chapter I explain the concepts of the machine.

### Memory
The machine has a memort, which is adressable trough an `u8` number (0-255), which are displayed in `HEX`. The first `0x0F` characters can be printed as of now, if you write to the later ones, they will be written and stored, but not displayed. Maybe I will add a way to display them later.
Every adress can hold an `u8` number (0-255). The memory is stored in a rust vector, which is a growable array. Every adress has the default value of `0`, but that can be set programmatically. The memory functions begin with `:mem` and are defined in the chapter about the scripting language.

### Source
The source, or code, is just like the memory, stored in a vector of `u8` numbers. The first `0x0F` can be printed as of now, if you write to the later ones, they will be written and stored, but neither displayed nor read, as the eye (next chapter) does not go further than `0x0F`. This will also hopefully change in the future. Now in every address you can store one line of code. The way the code works is a mess as of now and will need huge refactorings.
The code functions begin with `:src` and are defined in the chapter about the scripting language.

### Eye
The eye is our pointer and also interpreter, which points to a certain address in the code. It will "look" at a certain line of code and execute it if instructed to do so. The eye functions begin with `:eye` and are defined in the chapter about the scripting language. Another thing to keep in mind is, that the `:step` functions also have to do with the eye.

## Scripting language
In this chapter I will document the scripting language that I wrote for this. Is it touring complete? Absolutely not (yet). Is it at least half-way decent? Nope.

### How does it work?
The scripting language is kind of inspired by writing bash scripts. Every command that you can write onto the `:src` you can also execute manually, because the same interpreter is used for the command line and also for the eye.

### How do I use it?
The documentation about the `:src` command will teach you on how to actually write code and have it stored. If you want to execute code, you can either write it into the `:src` and then use the `:step` command to make the eye execute it, or you can write it into the command line and execute it there.

### :mem
If you pass this function no parameters it will print the memory up until the address `0x0F`.

#### :mem write <address> <value>
This function will write the value to the address. The address has to be a `u8` number (0-255) and the value has to be a `u8` number (0-255). The address will be displayed in `HEX`, but the user input happens in `DEC`.

#### :mem clear <address>
This function will clear the value at the address. The address has to be a `u8` number (0-255). The address will be displayed in `HEX`, but the user input happens in `DEC`.

### :src
If you pass this function no parameters it will print the source up until the address `0x0F`.

#### :src write <address> <value>
This function will write the value to the address. The address has to be a `u8` number (0-255) and the value has to be a `string`. That `string` can be split by as many whitecases as it wants. It will write everything after the address. The address will be displayed in `HEX`, but the user input happens in `DEC`.

#### :src clear <address>
This function will clear the value at the address. The address has to be a `u8` number (0-255). The address will be displayed in `HEX`, but the user input happens in `DEC`.

### :eye
If you pass this function no parameters it will print the current address of the eye.

### :step
If you pass this function no parameters it will execute the line of code at the current address of the eye and then step the eye one address down.

## Future work
Here everything that needs to be changed is written down.
### Change the concept of the programming language and implement a cache
Now the functions work kind of clunky. The goal is, to atomize every function as a standalone function that mutates the value in our cache. Certain functions like the one to get from memory dont need any previous value in cache, as they overwrite it. Other functions like maybe write(5) would need a value in the cache and write the cached value to memory addres 5. Like that the code starts becoming easier to work with etc.

Happy hacking :)

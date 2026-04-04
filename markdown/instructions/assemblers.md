# Assemblers

> Used to translate assembly into machine code

There are GAS, MASM, NASM, FASM, et cetera. Regarding the mentioned ones, MASM is an assembler from Microsoft, GAS is GNU's and integrates well with GCC, FASM has a good macro-system (my bias).

All of the assemblers output either machine code, or object files. Nowadays it's more common to write assembly that interacts with high-level languages, like C and Rust.

If you're picking an assembler for hand-writing, it's advisable to pick which has better macros (for you), instead of what has the best tooling integration. The reason is strictly because regardless of your tools, if you cannot reason properly, none of it matters, and macros help you to do so. They also help create the sense of indirection that functions in high-level languages provide. Thus reducing mental load. Given my experience, fasm has better macros.

If your priority is actually integrating with your tools, the assembler makes a huge difference in convenience. For example: GCC can compile GAS assembly, so you don't have to make your own pipeline of importing the object files to GCC. 

Also, worth noting that most of the mentioned assemblers are x86-64 specific — assemblers generally are arch-specific.

In the end, assembly is all the same if the ISA is the same, assemblers will differ little regarding syntax.

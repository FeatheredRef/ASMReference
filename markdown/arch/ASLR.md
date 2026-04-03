# ASLR

> Randomize memory space of instances, to make it harder for exploiters to hack through memory related vulnerabilities.

Most of things are in [memory](</arch/memory>), including the binary of your running code. That being said, without ASLR, an attacker could reliably change, for example, the pointer of a function call, thus making your program call something malicious without intending to.

ASLR works by randomizing the place in memory, if you start a program, the stack, allocated memory, they will be in whichever page the RAM can keep, regardless of the usage (it'll fetch available pages, obviously).

In conclusion, ASLR is a method that prevent memory-corruption attacks by making the placement on memory random.

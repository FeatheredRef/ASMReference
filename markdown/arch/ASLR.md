# ASLR

> Randomize memory space of instances, to make it harder for exploiters to hack through memory related vulnerabilities.

Most of things are in [memory](</arch/memory>), including the binary of your running code. That being said, without ASLR, an attacker could reliably change, for example, the pointer of a function call, thus making your program call something malicious without intending to.

ASLR works by randomizing the place in memory which something starts in, for instance, if you are using 1GiB of ram and you have 8GiB availabe, instead of outright allocating memory at the 1GiB, it'll be alocateb between it and the limit.

In conclusion, ASLR is a method that prevent memory-corruption attacks by making the placement on memory random.

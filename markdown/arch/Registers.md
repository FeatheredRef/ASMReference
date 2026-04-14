# Registers

Registers are for interacting with data in the chip; instructions use them to operate. There are general purpose registers and specific purpose registers, where general purpose are those which can be freely mutated and read, with no theoretical restriction while within its size. As for specific purpose registers, they are either hardwired into the processor (like x0 in Risc-V), or meant to serve a purpose. Like `RSP` in x86-64 which is the [stack](</arch/stack>) pointer. Which can have any value (that respects its size) written into it, but doing so will cause undefined behavior in the general case.

Some are made to serve the functionality of a feature efficiently. Like [SIMD](</arch/SIMD>) registers such as: `xmm`, `ymm`, and `zmm`. That are vector registers used in AVX, AVX2, and AVX512 in x86-64.

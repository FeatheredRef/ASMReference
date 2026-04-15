> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# FINIT

Initializes the x87 FPU by resetting the FPU control word to 384 (binary `00000010 00000000b`), clearing the status word, and emptying the ST(0) through ST(7) register stack.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| #I | #I |

DO NOT support LOCK

This instruction is available in 64-bit mode, compatibility mode, and protected mode. It does not require any operands and operates exclusively on the internal state of the x87 FPU.

Execution of FINIT clears all floating-point exception flags and resets the FPU stack pointer (TOP) to 0. Failure to initialize the FPU before performing x87 operations MAY result in an undefined state or incorrect precision handling if the control word was previously modified by another process.
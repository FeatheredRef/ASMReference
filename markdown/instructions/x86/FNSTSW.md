> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# FNSTSW

Stores the current x87 FPU status word into the specified destination.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| Internal FPU Status Word | reg (16-bit), m16 |

DO NOT support LOCK

This instruction is supported in 64-bit mode, but only when operating on the x87 FPU state. In x86-64, it is primarily used for compatibility with 32-bit floating point code.

The destination MUST be a 16-bit register or memory location. Attempting to use a larger register size may result in the upper bits remaining unchanged or cause unexpected behavior depending on the assembler's implementation of the opcode. To ensure portable state saving, the destination SHOULD be explicitly defined as a word.
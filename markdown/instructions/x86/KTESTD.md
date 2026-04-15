> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# KTESTD

Performs a logical AND between a 32-bit register and a 32-bit source (register or memory), updating the ZF flag based on the result without modifying the destination register.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| r32 | r32 |
| m32 | r32 |
| imm | #I |

DO NOT support LOCK

This instruction is available only in 64-bit mode. It is not supported in compatibility mode.

The destination register MUST be a general-purpose register; using a memory operand as the destination is prohibited. Because this instruction does not modify the destination operand, it is used exclusively for conditional branching based on bitmasks. Ensure the operand size is specifically 32-bit to avoid unexpected behavior with the upper 32 bits of a 64-bit register.
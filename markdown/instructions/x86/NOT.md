> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# NOT

Performs a bitwise logical NOT operation on the destination operand. The result is the one's complement of the source operand.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| mN | reg |
| reg | mN |
| mN | mN |
| #I | #I |

DO NOT support LOCK

The instruction is available in 16-bit, 32-bit, and 64-bit operand sizes. In 64-bit mode, if a 32-bit operand is used, the upper 32 bits of the destination register are NOT affected.

The NOT instruction does not affect any EFLAGS register bits. Users MUST NOT rely on the flags to determine the result of the operation.
> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PMOVSX

Moves a sign-extended value from a source operand to a destination operand. The instruction sign-extends a source of size 1, 2, or 4 bytes to a 64-bit destination.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| m1, m2, m4 | r64 |
| r8, r16, r32 | r64 |
| imm | #I |

DO NOT support LOCK

This instruction is only available in 64-bit mode. It is NOT supported in compatibility mode or 32-bit mode.

To avoid unexpected behavior, ensure the destination is always a 64-bit register, as any other destination size is invalid. When using memory sources, the memory operand MUST be properly aligned to its size to avoid potential performance penalties or alignment exceptions depending on the processor configuration.
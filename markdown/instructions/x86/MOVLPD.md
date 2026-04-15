> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# MOVLPD

Moves a 64-bit floating-point value from the source to the destination.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| m8 | #I |
| m16 | #I |
| m32 | #I |
| m64 | xmm |
| xmm | xmm |
| xmm | m64 |

DO NOT support LOCK

This instruction is available in 64-bit mode and 32-bit mode. In 64-bit mode, the memory operand must be 64-bit aligned to avoid performance penalties, although unaligned access is supported.

The instruction does not perform any floating-point calculations; therefore, it does not trigger any floating-point exceptions such as #D, #O, #U, or #P. Since it is a move operation, only the low 64 bits of the xmm registers are affected; the upper 64 bits of the destination xmm register remain unchanged.
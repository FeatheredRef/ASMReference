> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# MOVSHDUP

Copies the 64 bits of a source operand to a destination operand, then copies the high 32 bits of the source to the low 32 bits of the destination.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| m8 | #I |
| m16 | #I |
| m32 | #I |
| m64 | r64 |
| r64 | r64 |
| imm | #I |

DO NOT support LOCK

This instruction is ONLY available in 64-bit mode. It is NOT supported in compatibility mode.

The destination register MUST be a 64-bit register. If the source is a memory operand, it MUST be aligned to an 8-byte boundary to avoid performance penalties or faults depending on the alignment check flag (AC) in the EFLAGS register.

Ensure that the source operand is exactly 64 bits wide; attempting to use this instruction with 32-bit or smaller operands will result in an invalid opcode.
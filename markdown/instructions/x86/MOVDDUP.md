> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# MOVDDUP

Duplicates the low 64 bits of the source operand into the low and high 64 bits of the destination operand.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| m16 | #I |
| m32 | #I |
| m64 | #I |
| m128 | xmm |
| xmm | xmm |

DO NOT support LOCK

This instruction REQUIRES the SSE3 instruction set extension. It is available in 64-bit mode, compatibility mode, and 32-bit mode.

The destination operand MUST be an xmm register. If the source is a memory operand (m128), the memory access MUST be aligned to 16 bytes to avoid performance penalties or faults depending on the alignment check (AC) flag in EFLAGS. Use of a non-xmm register for the destination SHALL result in an invalid opcode exception.
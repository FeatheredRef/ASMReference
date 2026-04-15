> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# MOVHPD

Moves the high double-precision floating-point value from the source operand to the high double-precision floating-point value of the destination operand.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| m8 | #I |
| m16 | #I |
| m32 | #I |
| m64 | #I |
| m128 | xmm |
| xmm | xmm |
| imm | #I |

DO NOT support LOCK

This instruction SHALL only be used in 64-bit mode or 32-bit mode. It is not supported in 16-bit mode.

The destination operand MUST be an XMM register; memory destinations are NOT supported. To avoid unexpected behavior or faults, the memory source operand MUST be aligned on a 16-byte boundary unless the processor supports unaligned memory accesses (SSE2). Failure to align the memory source MAY result in a general-protection exception.
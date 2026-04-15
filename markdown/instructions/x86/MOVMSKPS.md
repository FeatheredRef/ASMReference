> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# MOVMSKPS

MOVMSKPS creates a bit mask from the sign bits of a packed single-precision floating-point vector. It extracts the most significant bit (the sign bit) from each of the four 32-bit floating-point elements in the source operand and stores the resulting 4-bit mask in the destination operand.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm reg | r32 reg |
| m128 | r32 reg |

DO NOT support LOCK

This instruction requires the SSE extension. It operates on XMM registers and is available in both 64-bit mode and compatibility mode.

The destination register is a general-purpose register (r32). Bits 4 through 31 of the destination register are zeroed. To avoid incorrect mask interpretation, the programmer MUST ensure the destination register is a 32-bit general-purpose register, as the instruction does not affect the upper 32 bits of a 64-bit register (r64) except by zeroing the upper bits of the target r32. Memory alignment of the source operand MUST follow the requirements for the specific memory access size to avoid alignment check exceptions or performance degradation.
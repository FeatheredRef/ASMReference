> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# BLENDVPD

BLENDVPD blends double-precision floating-point values from two source operands based on a mask provided by a third operand. For each 64-bit element, if the corresponding mask bit in the third operand is 1, the element from the second source operand is selected; otherwise, the element from the first source operand is selected.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm reg, xmm reg, xmm reg/m128 | xmm reg |

DO NOT support LOCK

This instruction is available only in 64-bit mode or compatibility mode when using XMM registers. It requires the AVX instruction set to be supported by the processor.

The mask operand is treated as a set of bits where only the least significant bit of each 64-bit lane is used to determine the selection. If the mask is provided via memory (m128), it MUST be aligned to the required boundary to avoid alignment check exceptions. Using an unaligned memory operand for the mask on older AVX implementations MAY result in a general protection fault (#GP) if alignment checking is enabled.
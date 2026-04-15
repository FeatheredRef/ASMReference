> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VINSERTF32x4

Inserts four single-precision floating-point values from the source operand into the destination operand. The insertion occurs at the index specified by the immediate operand.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| xmm reg | xmm reg |
| m128 | xmm reg |

DO NOT support LOCK

This instruction is available in both 64-bit mode and compatibility mode. It requires AVX support.

The immediate operand MUST be 0 or 1. An immediate value outside this range will result in an invalid instruction exception. If the destination is an xmm register and the immediate is 0, the lower 128 bits are replaced; if the immediate is 1, the upper 128 bits of a ymm register are targeted, but since the destination is constrained to xmm, this operation behaves according to the specific VEX encoding for 128-bit vectors. Using this instruction with a ymm destination requires the VEX prefix to specify the appropriate vector length.
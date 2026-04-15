> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# BLENDVPS

BLENDVPS blends packed single-precision floating-point values from two XMM source operands based on a mask provided by an immediate byte. For each 32-bit element, if the corresponding bit in the immediate mask is 0, the element from the first source operand is selected; if the bit is 1, the element from the second source operand is selected.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| xmm, xmm, imm | xmm |
| m128, xmm, imm | xmm |
| xmm, m128, imm | xmm |

DO NOT support LOCK

This instruction SHALL be executed in 64-bit mode or compatibility mode. It requires the SSE4.1 instruction set extension.

The immediate byte mask only utilizes the lower 4 bits to control the blending of the four 32-bit floating-point elements; the upper 4 bits of the immediate byte are ignored. Failure to align memory operands to 16-byte boundaries MAY result in a general-protection exception if aligned move instructions are used internally by the implementation, though BLENDVPS generally supports unaligned memory access.
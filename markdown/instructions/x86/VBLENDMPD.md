> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VBLENDMPD

Blends two double-precision floating-point vectors based on a mask specified by an immediate value. For each double-precision element, the instruction selects the corresponding element from either the first or second source operand according to the mask.

The following table specifies the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| xmm/ymm/zmm reg, xmm/ymm/zmm reg, imm | xmm/ymm/zmm reg |
| xmm/ymm/zmm reg, m128/m256/m512, imm | xmm/ymm/zmm reg |
| m128/m256/m512, xmm/ymm/zmm reg, imm | xmm/ymm/zmm reg |
| m128/m256/m512, m128/m256/m512, imm | xmm/ymm/zmm reg |

DO NOT support LOCK

This instruction SHALL be used only in 64-bit mode or compatibility mode. It requires the AVX extension; execution on a processor that does not support AVX SHALL result in an invalid opcode exception.

The mask is encoded as an immediate value; therefore, it SHALL be known at compile time. If the destination register is also one of the source registers, the original values are preserved until the operation completes. Ensure that the vector lengths of the source operands are compatible with the destination register size to avoid undefined behavior.
> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VEXPANDPS

Expands a packed single-precision floating-point vector by inserting zeros between the original elements. The instruction takes a source vector and a control immediate that specifies the expansion factor (2x, 4x, or 8x), placing the original elements at the start of each expanded group and filling the remaining slots with 0.0.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm/ymm/zmm reg | xmm/ymm/zmm reg |
| m32/m64/m128/m256/m512 | xmm/ymm/zmm reg |

DO NOT support LOCK

This instruction is only available in 64-bit mode or compatibility mode. It requires AVX-512 support (specifically AVX-512F). 

The destination register is overwritten. If the destination register is also used as a source, the operation is performed atomically on the register state.

The immediate operand MUST be a valid expansion factor (0, 1, or 2), representing expansion by 2, 4, or 8 elements respectively. Using an unsupported immediate value will result in an invalid opcode exception. When the expanded result exceeds the size of the destination register (e.g., expanding a zmm register by 8x), the elements beyond the register boundary are discarded.
> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VPTESTMD

Performs a logical AND operation between the low-order dword of the first operand and the low-order dword of the second operand. If the result is non-zero, the ZF flag is cleared; otherwise, the ZF flag is set.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm/ymm/zmm reg | flags |
| m32 | flags |

DO NOT support LOCK

This instruction is available only in 64-bit mode or compatibility mode. It requires the AVX instruction set.

To avoid unexpected behavior, ensure that only the lower 32 bits of the source registers are considered, as the upper bits of the registers are ignored for the purpose of the logical test. The instruction does not modify the source operands.
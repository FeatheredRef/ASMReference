> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VPERMW

Permutes 16-bit words from the first source operand according to the indices specified in the second source operand. For each word in the destination, the instruction selects a word from the first source operand based on the index provided in the corresponding word of the second source operand.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm/ymm/zmm reg | xmm/ymm/zmm reg |
| xmm/ymm/zmm reg | xmm/ymm/zmm reg |
| #I | #I |

DO NOT support LOCK

This instruction is available only in 64-bit mode or 32-bit mode; it is NOT supported in compatibility mode. It requires the AVX-512BW extension to be enabled.

The index specified in the second source operand MUST be within the range of the number of words available in the first source operand. Indices that are out of range SHALL result in a zero value being written to the corresponding word of the destination register. If the destination register is the same as one of the source registers, the operation is performed as if the sources were copied to temporary registers first.
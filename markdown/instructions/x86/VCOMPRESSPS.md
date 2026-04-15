> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VCOMPRESSPS

Compresses selected single-precision floating-point values from a source vector to a destination vector based on a mask. Values from the source are copied to the destination in the order they appear, but only if the corresponding bit in the mask register is set.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm/ymm/zmm reg | xmm/ymm/zmm reg |
| m32/m64/m128/m256/m512 | xmm/ymm/zmm reg |

DO NOT support LOCK

This instruction is available only in 64-bit mode or compatibility mode. It requires the AVX-512 foundation (AVX512F) and the AVX-512 BW extension for specific vector lengths.

The destination register is updated only for the elements that are compressed. Elements in the destination register beyond the number of set bits in the mask remain unchanged. If the mask is all zeros, the destination register is not modified. Users MUST ensure that the mask register used is compatible with the vector length of the operands to avoid undefined behavior.
> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VPCOMPRESSD

Compresses a vector of dwords from a source vector to a destination vector based on a mask. For each bit set in the mask, the corresponding dword from the source is packed into the destination vector at the next available position, moving from the lowest index to the highest.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm/ymm/zmm reg | xmm/ymm/zmm reg |
| m32/m64/m128 | xmm/ymm/zmm reg |

DO NOT support LOCK

This instruction REQUIRES the AVX512VL and AVX512F instruction sets. It is ONLY available in 64-bit mode and 32-bit mode; it is NOT supported in compatibility mode.

To avoid undefined behavior or data corruption, ensure that the destination register is not also used as the source register unless the implementation specifically supports overlapping operands for this instruction. When using a mask, elements not selected by the mask are not written to the destination, and the remaining elements of the destination register are preserved or zeroed depending on the masking mode (merging vs. zeroing). Failure to account for the mask's effect on the destination register index may lead to incorrect data placement.
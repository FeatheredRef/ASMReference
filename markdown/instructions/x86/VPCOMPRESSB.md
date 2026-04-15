> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VPCOMPRESSB

Compresses bytes from a source vector to a destination vector based on a mask. For each bit set in the mask, the corresponding byte from the source is copied to the destination, packing them contiguously starting from the lowest element index.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm/ymm/zmm reg | xmm/ymm/zmm reg |
| m1 | xmm/ymm/zmm reg |

DO NOT support LOCK

This instruction requires the AVX512-VBMI subset. It is only available in 64-bit mode or compatibility mode.

The destination register is partially overwritten; elements beyond the number of set bits in the mask remain unchanged. To avoid data corruption or unexpected results when reusing the destination register, the user SHALL ensure the destination is either cleared or the range of affected elements is known. If the mask is all zeros, no elements are modified in the destination register.
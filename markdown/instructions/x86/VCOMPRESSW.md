> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VCOMPRESSW

VCOMPRESSW compresses words from a source vector register to a destination vector register based on a mask. For each bit set in the mask, the corresponding word from the source is copied to the destination, packing them contiguously starting from the lowest index.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm/ymm/zmm | xmm/ymm/zmm |

DO NOT support LOCK

This instruction is only available in 64-bit mode. It requires support for the AVX-512 extension and specifically the AVX-512 BW (Byte and Word) foundation.

The destination register is updated only for the elements specified by the mask; elements not targeted by the mask remain unchanged (merging masking). If the destination and source registers are the same, the operation is performed in-place. Users SHALL ensure that the mask register contains the correct bitmask to avoid unintended data overwrites or loss of elements during the compression process.
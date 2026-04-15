> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VPMOVWB

Moves each 8-bit byte from a source to a 16-bit word in a destination, with zero-extension.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |
| ymm | ymm |
| zmm | zmm |
| m1 | xmm |
| m1 | ymm |
| m1 | zmm |

DO NOT support LOCK

This instruction is available only in 64-bit mode. It requires the AVX-512 BW foundation. Use of this instruction in 32-bit mode or without the appropriate CPU feature flags SHALL result in an invalid opcode exception.

The destination register is overwritten. If the destination is the same as the source, the behavior is defined by the zero-extension of the 8-bit elements into 16-bit lanes, effectively shifting the data layout. Failure to account for the doubling of the data size in the destination register MAY lead to out-of-bounds memory access when storing the resulting register to memory.
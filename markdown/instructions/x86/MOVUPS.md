> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# MOVUPS

Moves 128 bits of packed single-precision floating-point data from a source to a destination. This instruction does not require the memory addresses to be aligned on 16-byte boundaries.

The table below covers the supported sources and destinations.

| source | destination(s) |
| :--- | :--- |
| xmm reg | xmm reg |
| m128 | xmm reg |
| xmm reg | m128 |

DO NOT support LOCK

This instruction is available in 64-bit mode and 32-bit mode. It requires the SSE extension to be supported by the processor.

Using `MOVUPS` on memory that is aligned to a 16-byte boundary may result in better performance on older microarchitectures, although modern processors minimize the performance penalty difference between `MOVUPS` and `MOVAPS`. Failure to use `MOVUPS` when data is unaligned will result in a general-protection exception (#GP).
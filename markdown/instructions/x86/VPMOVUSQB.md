> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VPMOVUSQB

Moves unsigned bytes from a source to a destination and sign-extends them to 32-bit signed integers.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm / ymm / zmm reg | xmm / ymm / zmm reg |
| m8 | xmm / ymm / zmm reg |

DO NOT support LOCK

This instruction is only available in 64-bit mode or compatibility mode. It requires the AVX-512 BW foundation extension to be supported by the processor.

The destination register MUST be the same size as the source if both are registers. If the source is a memory operand, the destination register size determines the number of bytes read from memory. Failure to align memory accesses to the vector width MAY result in performance degradation, though it is not functionally required for correctness.
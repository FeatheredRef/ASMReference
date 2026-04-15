> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VPMOVDW

Moves double words to words. This instruction converts a packed 128-bit or 256-bit vector of 32-bit signed integers to a packed vector of 16-bit signed integers by saturating the values to the range of a signed word.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| xmm/ymm | xmm/ymm |
| m16/m32 | xmm/ymm |

DO NOT support LOCK

This instruction is available only in 64-bit mode or compatibility mode. It requires the AVX instruction set extensions.

The destination register is overwritten. If the source is a memory operand, it MUST be aligned to the size of the vector to avoid performance penalties or alignment exceptions depending on the CPU configuration. Since this is a saturating operation, any value exceeding the range of a signed 16-bit integer (i16) will be clamped to the maximum or minimum representable value of i16.
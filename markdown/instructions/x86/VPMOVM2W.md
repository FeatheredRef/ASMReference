> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VPMOVM2W

Moves a packed 32-bit integer value from a source to a destination, truncating the value to a packed 16-bit integer.

The following table covers the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| xmm/ymm/zmm | xmm/ymm/zmm |
| m128/m256/m512 | xmm/ymm/zmm |

DO NOT support LOCK

This instruction is available in 64-bit mode and 32-bit mode. It requires the AVX-512 BW instruction set extension.

The destination register is overwritten. If the source and destination are the same register, the operation is performed in-place. Ensure that the target register is of the same vector length as the source to avoid undefined behavior or faults.
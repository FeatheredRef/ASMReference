> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VGETEXPPD

Extracts the exponent of the packed double-precision floating-point values from the source operands and stores the result as signed integers in the destination operand.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm/ymm/zmm register | xmm/ymm/zmm register |
| m64/m128/m256/m512 | xmm/ymm/zmm register |

DO NOT support LOCK

This instruction SHALL only be executed in 64-bit mode or compatibility mode. It is part of the AVX-512 extension set; therefore, it REQUIRES a processor that supports AVX-512.

The result is stored as a signed 32-bit integer for each 64-bit floating-point element. Users MUST ensure that the destination register is of sufficient size to hold the resulting packed 32-bit integers to avoid data truncation or unexpected behavior. When using masking (k-registers), elements that are masked out SHALL retain their original values in the destination register.
> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VPSHLDV

Shifts the doublewords in each packed 128-bit element of the first source operand to the left by the count specified in the second source operand. The shifted-out bits are filled with zeros, and the shift count is masked to u5.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm/ymm/zmm | xmm/ymm/zmm |
| xmm/ymm/zmm | xmm/ymm/zmm |
| #I | #I |

DO NOT support LOCK

This instruction is available only in 64-bit mode or compatibility mode. It requires AVX-512DQ support.

The shift count is masked to u5, meaning only the low-order 5 bits of the count operand are used; any bits beyond bit 4 are ignored. Failure to mask the count manually before execution is unnecessary but failure to account for this masking in logic may lead to unexpected shift distances.
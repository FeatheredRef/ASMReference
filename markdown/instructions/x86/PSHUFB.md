> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PSHUFB

The instruction shuffles a doubleword or quadword by using a byte mask. For each byte in the destination, the instruction selects a byte from the source based on the corresponding index byte in the mask; if the most significant bit of the index byte is set, the resulting byte is zeroed.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm / ymm / zmm reg | xmm / ymm / zmm reg |
| xmm / ymm / zmm m16 / m32 / m64 | xmm / ymm / zmm reg |

DO NOT support LOCK

This instruction SHALL only be executed in 64-bit mode or compatibility mode when using XMM, YMM, or ZMM registers. It requires the SSE4.1 instruction set extension (for XMM) or AVX/AVX2/AVX-512 (for YMM/ZMM).

The index byte is treated as an u8. Only the low 4 bits of the index byte are used to select the source byte. If the index byte is greater than or equal to the number of bytes in the source operand, the result is undefined. The destination register SHALL be cleared of any data beyond the vector length of the source operand if using specific AVX-512 masking variants.
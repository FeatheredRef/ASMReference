> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PMULHRSW

Multiplies the signed packed 16-bit integers in the source operand by the signed packed 16-bit integers in the destination operand. The result is then shifted right by 16 bits, and the lower 16 bits of the product are discarded. The result is stored in the destination operand.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |
| m128 | xmm |

DO NOT support LOCK

This instruction is available in 64-bit mode and 32-bit mode. It requires the SSE4.1 instruction set extension.

The multiplication is performed as a signed 16-bit integer multiplication. The result of the multiplication is a 32-bit signed integer. The right shift by 16 bits is an arithmetic shift, meaning the sign bit is preserved. Because the result is shifted right by 16 bits, the operation effectively calculates $\lfloor (a \times b) / 2^{16} \rfloor$. User MUST ensure the target processor supports SSE4.1 to avoid an invalid opcode exception.
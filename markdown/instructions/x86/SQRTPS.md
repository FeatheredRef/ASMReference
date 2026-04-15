> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# SQRTPS

Computes the square root of four single-precision floating-point values. Each value in the source operand is processed independently and the result is stored in the destination operand.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |
| m128 | xmm |

DO NOT support LOCK

This instruction requires SSE3 support. It is available in both 64-bit mode and compatibility mode.

If the input value is a negative number (excluding negative zero), a #I exception is generated and the resulting value is a NaN. If the input is negative zero, the result is negative zero. Precision (#P) and Denormal (#D) exceptions may be triggered depending on the MXCSR register settings.

The destination register must not be the same as the source memory operand. Ensure the memory operand is 16-byte aligned to avoid alignment check exceptions or performance penalties.
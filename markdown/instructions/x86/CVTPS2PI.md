> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# CVTPS2PI

Converts the low 32 bits of a packed single-precision floating-point value to a packed signed 32-bit integer using truncation. The result is stored in the destination.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| m128 | xmm |
| xmm | xmm |

DO NOT support LOCK

This instruction requires SSE4.1 support. It operates on the low 32 bits of the XMM registers; the upper 96 bits of the destination register are zeroed.

The conversion uses the truncation method (rounding toward zero). If the source value is NaN or the resulting integer is too large to be represented as an i32, the destination is set to the integer indeterminate value (0x80000000). An #O exception is generated in this case. Precision #P is generated if the result is not exact.
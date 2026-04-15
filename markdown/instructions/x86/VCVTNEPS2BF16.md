> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VCVTNEPS2BF16

Converts packed single-precision floating-point values to packed bfloat16 floating-point values. The conversion is performed using a rounding mode specified by the immediate operand or the MXCSR register.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |
| m128 | xmm |

DO NOT support LOCK

This instruction is available only in 64-bit mode. It requires the AVX-512 BF16 extension.

The rounding mode is controlled by the immediate byte; if the immediate is not specified, the rounding control field in the MXCSR register SHALL be used. Failure to ensure the correct rounding mode may result in unexpected precision loss. Precision (#P) may be signaled if the result of the conversion is inexact.
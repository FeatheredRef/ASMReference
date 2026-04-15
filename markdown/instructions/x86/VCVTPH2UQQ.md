> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VCVTPH2UQQ

Converts packed half-precision floating-point values to unsigned quadword integers. The instruction converts each half-precision value in the source to an unsigned 64-bit integer using the rounding mode specified in the MXCSR register.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| m128 | xmm |
| xmm | xmm |

DO NOT support LOCK

This instruction is only available in 64-bit mode. It requires AVX-512 and the AVX-512 FP16 extension.

The conversion follows the rounding mode set in the MXCSR register. If the value is NaN, the result is the indeterminate value. If the value is too large to be represented as a u64, the result is the maximum representable u64 value, and a #O exception is generated. Precision loss during conversion triggers a #P exception.
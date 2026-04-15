> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VCVTSI2SH

Converts a signed integer value to a packed signed 16-bit floating-point value (half-precision) and stores the result in the destination.

The following table specifies the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| xmm (i32) | xmm (f16) |

DO NOT support LOCK

This instruction is available only in 64-bit mode and 32-bit mode. It requires the AVX-512 FP16 extension.

The conversion follows the IEEE 754 standard for half-precision floating-point numbers. If the source integer value is too large to be represented in f16, the operation SHALL result in #O and the destination SHALL be set to the corresponding infinity. If the result cannot be represented exactly, the value SHALL be rounded according to the current rounding mode in MXCSR, which MAY trigger #P.
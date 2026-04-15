> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VCVTDQ2PH

Converts packed signed 64-bit integers to packed half-precision floating-point numbers. The conversion is performed using the rounding mode specified in the MXCSR register.

The following table describes the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| xmm / ymm / zmm reg | xmm / ymm / zmm reg |

DO NOT support LOCK

This instruction is available only in 64-bit mode. It requires the AVX-512 FP16 extension to be supported by the processor.

The instruction performs a conversion from `i64` to `f16`. If the signed integer value is too large to be represented as a half-precision floating-point number, the result is determined by the rounding mode; it MAY result in #O. Precision loss resulting in an inexact result SHALL trigger #P. The source register must be a floating-point register containing signed 64-bit integers.
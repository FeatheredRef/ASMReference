> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VCVTPS2PH

Converts packed single-precision floating-point values to packed half-precision floating-point values according to the rounding mode specified in the immediate operand.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |
| m128 | xmm |

DO NOT support LOCK

This instruction is only available in 64-bit mode and 32-bit mode. It requires the AVX-512 FP16 extension.

The rounding mode is controlled by the immediate operand; if the immediate is not a valid rounding mode, the instruction shall trigger an invalid operand exception. If the rounding mode is not specified via the immediate, the rounding is governed by the MXCSR register.

Failure to provide an xmm register as a destination will result in a compile-time error or an illegal instruction exception. Precision loss is expected when converting from f32 to f16, which SHALL trigger #P if the result is inexact. Numeric overflow resulting in infinity SHALL trigger #O.
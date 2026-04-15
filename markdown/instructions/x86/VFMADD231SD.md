> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VFMADD231SD

Multiplies a scalar double-precision floating-point value from a source operand by a scalar double-precision floating-point value from a second source operand, adds the result to a scalar double-precision floating-point value from a third source operand, and stores the result in the destination.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg, m8 | reg |

DO NOT support LOCK

This instruction is only available in 64-bit mode and 32-bit mode; it is NOT supported in compatibility mode. It requires the FMA3 extension to be enabled.

The destination register MUST NOT be the same as the memory operand to avoid aliasing issues. The operation is performed according to the IEEE 754 standard, meaning it utilizes a single rounding step at the end of the fused multiply-add operation to maintain higher precision.

If the result exceeds the maximum representable value for f64, the #O exception SHALL be raised. If the result is an inexact representation, the #P exception SHALL be raised. Operations involving signaling NaNs SHALL trigger #I.
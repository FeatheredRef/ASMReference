> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VCVTPH2DQ

Converts packed half-precision floating-point values to packed signed 64-bit integers. The conversion is performed by rounding to the nearest integer using the rounding-control mode currently set in the MXCSR register.

The following table describes the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| m128 | xmm |
| xmm | xmm |

DO NOT support LOCK

This instruction is only available when the processor is operating in 64-bit mode or compatibility mode. It requires the AVX-512 FP16 instruction set extension.

If the converted value is too large to be represented as an i64, the result is the maximum or minimum representable signed 64-bit integer. If the input is a NaN, the result is the integer minimum value (i64 min). These operations MAY trigger #O or #P based on the rounding and precision of the conversion.
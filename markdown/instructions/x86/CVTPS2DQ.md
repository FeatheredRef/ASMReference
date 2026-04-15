> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# CVTPS2DQ

Converts packed single-precision floating-point values from the source to packed signed integers of type doubleword in the destination. The conversion is performed by truncating the floating-point value (rounding toward zero).

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |
| m128 | xmm |

DO NOT support LOCK

This instruction is available in 64-bit mode and 32-bit mode. It requires SSE4.1 support.

If the converted value is too large to be represented as an i32, the result is the integer value corresponding to the maximum or minimum representable signed 32-bit integer (saturation), and an #O exception is raised if the MXCSR exception mask is not set. If the input is a NaN, the result is the integer value corresponding to the indefinite integer value. This instruction may trigger #D, #O, and #P exceptions depending on the input values and MXCSR settings.
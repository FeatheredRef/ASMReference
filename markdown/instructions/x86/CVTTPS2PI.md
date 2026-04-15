> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# CVTTPS2PI

Converts packed single-precision floating-point values to packed signed integers with truncation. Each f32 value in the source is truncated toward zero to the nearest i32 value.

The following table specifies the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| xmm reg | xmm reg |
| m128 | xmm reg |

DO NOT support LOCK

This instruction is available only in 64-bit mode or compatibility mode. It requires the SSE4.1 instruction set extension.

If the converted value is too large to fit in the destination i32, the result is the indefinite integer value (0x80000000). This occurs when the value exceeds the range of a signed 32-bit integer, triggering an #O exception.

The instruction uses the MXCSR rounding mode for some floating-point operations, but specifically performs truncation regardless of the current rounding mode setting. Precision exceptions (#P) may be raised if the result is not exact.
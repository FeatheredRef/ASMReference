> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# CVTTPD2DQ

Converts the packed double-precision floating-point values to packed signed integers with 64-bit truncate. The instruction converts two f64 values from a source to two i64 values in a destination.

The following table describes the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |
| m16 | xmm |

DO NOT support LOCK

This instruction requires SSE4.1 support. It operates on XMM registers and is available in both 64-bit mode and compatibility mode.

The conversion uses the truncation rounding mode regardless of the current rounding control in MXCSR. If the floating-point value is NaN or exceeds the range of a signed 64-bit integer, the result is the integer minimum value (-9,223,372,036,854,775,808). This behavior ensures that overflow is handled by saturation to the minimum possible signed 64-bit integer.

The instruction may trigger the following floating-point exceptions:
- #O: If the result is too large to be represented in the destination format.
- #P: If the result is inexact.
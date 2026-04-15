> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# CVTTSS2SI

Converts a scalar single-precision floating-point value to a signed integer using truncation toward zero.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| f32 | r32 |
| m4 | r32 |

DO NOT support LOCK

This instruction is available in 32-bit and 64-bit mode. It is NOT supported in 16-bit mode.

If the source value is NaN, an #I exception is signaled and the destination is set to the integer indeterminate value. If the converted value is outside the range of a signed 32-bit integer (i32), an #O exception is signaled and the destination is set to the integer indeterminate value.

The destination register is updated with the truncated result. If the MXCSR register's rounding mode is not ignored by the truncation operation, the #P exception may be signaled if the result is inexact.
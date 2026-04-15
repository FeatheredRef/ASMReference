> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# CVTTSD2SI

Converts a scalar double-precision floating-point value to a signed integer using the truncation rounding mode.

The following table specifies the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| f64 (xmm) | r32 / m32 |
| f64 (xmm) | r64 / m64 |

DO NOT support LOCK

This instruction is available in 64-bit mode and compatibility mode. The destination operand size (dword or qword) determines the target integer size.

If the source value is NaN, the destination is set to the indefinite integer value. If the converted value is too large to fit in the destination operand (overflow), the destination is set to the indefinite integer value. 

The result of the conversion is subject to the truncation rounding mode regardless of the current rounding control in the MXCSR register. If the conversion results in a value that cannot be exactly represented as an integer, the #P (Inexact result) exception is raised.
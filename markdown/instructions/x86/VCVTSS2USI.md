> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VCVTSS2USI

Converts a scalar single-precision floating-point value to an unsigned integer. The instruction truncates the value (rounds toward zero).

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| f32 | r32 |
| f32 | r64 |
| m32 (f32) | r32 |
| m32 (f32) | r64 |

DO NOT support LOCK

This instruction is available only in 64-bit mode or compatibility mode. It requires the SSE3 extension set.

If the source value is NaN, the destination is set to the maximum unsigned integer value for the destination register size; the #I exception is raised. If the source value is too large to fit in the destination register (overflow), the destination is set to the maximum unsigned integer value; the #O exception is raised. If the source value is negative, the destination is set to 0; the #O exception is raised. Precision is handled via the #P exception if the result is inexact.
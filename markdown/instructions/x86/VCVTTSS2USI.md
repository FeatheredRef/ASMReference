> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VCVTTSS2USI

Converts a scalar single-precision floating-point value to an unsigned 32-bit integer using the truncation rounding mode.

The following table describes the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| f32 | r32 |
| m4 | r32 |

DO NOT support LOCK

This instruction is available in 64-bit mode and compatibility mode. It requires the SSE4.1 instruction set extension.

The result of the conversion is truncated toward zero. If the source value is NaN, infinity, or exceeds the range of a u32, the destination register is set to the indefinite integer value (0x80000000h). This operation does not trigger floating-point exceptions such as #O or #P.
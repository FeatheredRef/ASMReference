> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# CVTSS2SI

Converts a scalar single-precision floating-point value to a signed integer. The conversion is performed by truncating the fraction (rounding toward zero).

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| f32 | r32 |
| f32 | r64 |
| m32 (f32) | r32 |
| m32 (f32) | r64 |

DO NOT support LOCK

This instruction requires SSE3 support. In 64-bit mode, the destination register can be either a 32-bit or 64-bit general-purpose register. If a 32-bit register is specified, the result is truncated to i32; if a 64-bit register is specified, the result is sign-extended to i64.

If the converted value is too large to be represented in the destination integer type, the instruction SHALL signal #O and the destination register SHALL be set to the indefinite integer value (maximum or minimum representable value for the target size, depending on the sign). If the source is a Signaling NaN, #I SHALL be signaled. Precision #P is signaled if the result is rounded.
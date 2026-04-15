> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VCVTTPS2QQ

Converts packed single-precision floating-point values to packed signed 64-bit integers with truncation. The instruction converts each f32 element of the source to an i64 element in the destination.

The following table describes the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| m128 | xmm |
| xmm | xmm |

DO NOT support LOCK

This instruction is only available in 64-bit mode. It requires the AVX-512 Foundation (F) instruction set.

The conversion uses truncation toward zero. If the source value is NaN, the result is the integer INDEF own value. If the result of the conversion is too large to be represented in an i64, the result is the integer minimum or maximum value (saturation).

When utilizing the embedded broadcasting operand (if applicable to the specific encoding), a single f32 value from memory is broadcasted to all elements of the destination register before conversion. Failure to account for the destination register size (which is 256 or 512 bits for the result of the i64 conversion) may lead to unintended data overwrite in adjacent register lanes.

Floating-point exceptions:
- #I: Invalid operation if the source value is a signaling NaN.
- #P: Inexact result if the rounded result differs from the exact value.
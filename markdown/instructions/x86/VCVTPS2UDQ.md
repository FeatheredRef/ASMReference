> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VCVTPS2UDQ

Converts packed single-precision floating-point values to packed unsigned 64-bit integers with rounding. The instruction converts each f32 element in the source to a u64 integer according to the rounding mode specified in the MXCSR register.

The following table specifies the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |
| m128 | xmm |

DO NOT support LOCK

This instruction is available only in 64-bit mode. It requires the AVX-512 foundation extensions (specifically AVX-512F).

If the converted value is too large to be represented in a u64 integer, the result is the maximum representable unsigned 64-bit integer (2^64 - 1). If the source value is NaN, the result is undefined. The instruction triggers #O if the result exceeds the representable range of the destination type.
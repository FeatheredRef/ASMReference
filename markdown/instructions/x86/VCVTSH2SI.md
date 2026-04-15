> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VCVTSH2SI

Converts the signed 16-bit floating-point values (half-precision) from a source to signed integers in the destination. The conversion is performed using the truncation rounding mode (round-toward-zero).

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm/ymm/zmm | xmm/ymm/zmm |

DO NOT support LOCK

This instruction is available in 64-bit mode and 32-bit mode. It requires the AVX or AVX-512 instruction sets, depending on the register width used.

If the source value is a NaN, the destination is set to the integer minimum value (i32 min). If the source value is too large to fit in a signed 32-bit integer, the result is the integer minimum value (i32 min). Floating-point exceptions #O and #P may be raised depending on the rounding mode and result.
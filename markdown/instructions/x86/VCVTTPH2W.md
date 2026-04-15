> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VCVTTPH2W

Converts packed half-precision floating-point values to packed signed integers with truncation. Each f16 value in the source is truncated toward zero to the nearest i16 integer.

The table below covers the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |

DO NOT support LOCK

This instruction is available only in 64-bit mode. It requires the AVX-512 F a subset for execution.

The conversion process follows the truncation rule where the fractional part is discarded. If the result exceeds the range of a signed 16-bit integer, the result is the "integer indefinite" value (0x8000). Precision exceptions (#P) may be triggered if the source value cannot be represented exactly as an integer.
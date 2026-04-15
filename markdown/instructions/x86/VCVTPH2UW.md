> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VCVTPH2UW

Converts half-precision floating-point values (f16) to unsigned 16-bit integers (u16) with truncation. The instruction converts packed f16 values from the source to packed u16 values in the destination.

The following table covers the supported source and destinations.

| source | destination(s) |
| :--- | :--- |
| xmm/ymm/zmm reg | xmm/ymm/zmm reg |

DO NOT support LOCK

This instruction is available only in 64-bit mode or compatibility mode. It requires the AVX-512 FP16 instruction set extension.

The conversion uses the "truncate" rounding mode regardless of the current MXCSR rounding mode setting. If the result of the conversion is too large to be represented as a u16, the result is the maximum representable u16 value (65535). If the result is too small or NaN, the result is 0. Values should be checked against the architectural limits of u16 to avoid silent clamping.
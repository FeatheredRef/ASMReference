> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# CVTTPS2DQ

Converts packed single-precision floating-point values to packed signed integers with quadwords using truncation.

The following table specifies the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| m128 | xmm |
| xmm | xmm |

DO NOT support LOCK

This instruction is only available in 64-bit mode or compatibility mode. It requires the AVX instruction set extension to be supported by the processor.

The conversion uses truncation toward zero. If the source value is NaN, the result is the integer minimum value (i64 minimum). If the source value is too large to be represented as a signed 64-bit integer, the result is the integer minimum value (i64 minimum).

The destination register is overwritten by the resulting i64 values; however, since the source consists of four f32 values and the destination consists of two i64 values, only the lower 128 bits of the destination xmm register are updated. Specifically, the first two f32 elements of the source are converted to the two i64 elements of the destination. The upper two f32 elements of the source are ignored.

#P may be signaled.
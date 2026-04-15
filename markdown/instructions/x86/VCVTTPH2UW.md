> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VCVTTPH2UW

Convert packed half-precision floating-point values to packed unsigned words with truncation.

The following table specifies the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| m128 | xmm |
| xmm | xmm |

DO NOT support LOCK

This instruction is available only in 64-bit mode. It requires AVX support.

To avoid unexpected behavior, be aware that the instruction performs truncation toward zero. If the value is NaN, the result is set to the maximum representable unsigned word value. If the value exceeds the range of a 16-bit unsigned integer, the result is saturated to the maximum representable unsigned word value.
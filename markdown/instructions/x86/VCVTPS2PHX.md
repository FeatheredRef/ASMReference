> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VCVTPS2PHX

Convert packed single-precision floating-point values to packed half-precision floating-point values with a specified rounding mode.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| f32 (reg/m32) | f16 (reg) |

DO NOT support LOCK

This instruction is available only in 64-bit mode. It requires the AVX-512 FP16 extension.

The immediate operand MUST specify a valid rounding mode. If the immediate is not a valid rounding mode encoding, the instruction results in an #I. The conversion follows the specified rounding mode regardless of the MXCSR register settings. Precision exceptions (#P) may be triggered depending on the rounding result.
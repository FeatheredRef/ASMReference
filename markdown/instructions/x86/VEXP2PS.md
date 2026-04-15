> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VEXP2PS

Converts single-precision floating-point values to their exponential form (2^x). Each packed single-precision floating-point element of the first source operand is converted and the result is stored in the destination operand.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| xmm-reg | xmm-reg |
| m128 | xmm-reg |

DO NOT support LOCK

This instruction is only available in 64-bit mode or compatibility mode. It requires the AVX instruction set extension.

The instruction is subject to the floating-point control word settings. If the result of the operation is too large to be represented as a single-precision floating-point number, it SHALL trigger #O. If the result is too small, it SHALL trigger #U. Precision exceptions (#P) occur if the result is rounded. All operations must follow the IEEE 754 standard for floating-point arithmetic.
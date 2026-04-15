> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VFNMSUB132PS

Subtracts the product of the second source operand and the third source operand from the first source operand, then negates the result. The operation is performed on packed single-precision floating-point values.

The following table specifies the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| xmm / m32 | xmm |

DO NOT support LOCK

This instruction is only available in 64-bit mode or 32-bit mode. It REQUIRES the AVX CPU feature flag to be enabled.

The operation is subject to the floating-point control word settings. Results may trigger floating-point exceptions including #D, #O, #U, and #P depending on the rounding mode and the magnitude of the operands. Since this is a fused multiply-subtract operation, it performs the multiplication and subtraction with a single rounding step at the end, which minimizes precision loss compared to discrete instructions.
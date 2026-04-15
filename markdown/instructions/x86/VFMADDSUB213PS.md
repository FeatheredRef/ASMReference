> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VFMADDSUB213PS

Performs a fused multiply-add or multiply-subtract operation on packed single-precision floating-point values. It computes the result of $(a \times b) + c$ or $(a \times b) - c$ depending on the immediate operand, where the destination register is used as one of the source operands.

The following table specifies the supported source and destination operand types.

| source | destination(s) |
| :--- | :--- |
| reg, m32 | reg |

DO NOT support LOCK

This instruction requires the AVX target. It is available in 64-bit mode and 32-bit mode. It SHALL NOT be used in compatibility mode if the AVX state is not enabled.

The instruction uses a three-operand non-destructive form (213), meaning the destination register is the first operand (a), the second source is the second operand (b), and the third source is the third operand (c). If the destination register is also used as a source, the original value is preserved until the calculation is complete.

The operation is subject to floating-point exceptions: #P, #O, #U, #D, and #Z (if applicable). To avoid precision loss or unexpected results, the programmer MUST ensure that the MXCSR register is correctly configured for rounding modes. Since this is a fused operation, only one rounding step occurs at the end of the calculation, which reduces the accumulation of rounding errors compared to discrete multiply and add/subtract instructions.
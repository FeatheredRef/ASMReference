> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VRSQRT28PS

Computes the approximate reciprocal square root of each single-precision floating-point element in the source operand with a relative error of less than $2^{-12}$.

The following table specifies the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| xmm/ymm/zmm reg | xmm/ymm/zmm reg |
| m32/m64/m128/m256/m512 | xmm/ymm/zmm reg |

DO NOT support LOCK

This instruction is available only in 64-bit mode and compatibility mode. It requires AVX-512 support; specifically, it is part of the AVX-512 Foundation instructions.

The instruction does not support memory-to-memory operations. The destination MUST be a register.

Precision and accuracy of the result are limited; this instruction provides an approximation. For higher precision, a Newton-Raphson iteration SHOULD be performed using the result of this instruction as an initial guess.

Floating-point exceptions:
- #I: Occurs if the source operand is a NaN or $\pm\infty$.
- #P: Occurs if the result is inexact.
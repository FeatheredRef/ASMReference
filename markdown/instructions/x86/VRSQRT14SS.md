> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VRSQRT14SS

Computes the approximate reciprocal square root of a scalar single-precision floating-point value with a relative error of less than or equal to $2^{-14}$.

The following table describes the supported source and destination operands:

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |
| m4 | xmm |

DO NOT support LOCK

This instruction is available only in 64-bit mode. It requires the AVX instruction set to be enabled.

The result is an approximation; it SHALL NOT be used for high-precision calculations without subsequent refinement (e.g., using Newton-Raphson iterations). The instruction DOES NOT support the `LOCK` prefix. If the input is a Signaling NaN, the instruction SHALL trigger #I. If the input is $\pm 0$ or $\pm \infty$, the result is $0$. If the input is a Quiet NaN, the result is a Quiet NaN.

The MXCSR register controls the rounding mode and exception handling. Inexact results SHALL trigger #P.
> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VRSQRT14PS

Computes the approximate reciprocal square root of each single-precision floating-point value in a source operand with a relative error of $2^{-14}$.

The following table describes the supported sources and destinations.

| source | destination(s) |
| :--- | :--- |
| xmm/ymm/zmm reg | xmm/ymm/zmm reg |
| m32/m64/m128/m256/m512 | xmm/ymm/zmm reg |

DO NOT support LOCK

This instruction requires the AVX-512 foundation extensions. It is not supported in compatibility mode. The instruction behavior is subject to the MXCSR register settings, specifically regarding rounding control and exception masking.

To avoid precision issues, it MUST be noted that this instruction provides an approximation; it SHALL NOT be used where high-precision results are required without subsequent Newton-Raphson iterations. If the input is a NaN or $\pm\infty$, the result is NaN. If the input is $0$, the result is $\infty$.

The following exceptions may be raised:
- #P: Result is inexact.
- #O: Numeric overflow.
- #U: Numeric underflow.
- #D: Denormalized operand.
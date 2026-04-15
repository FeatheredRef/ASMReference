> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VRSQRT14PD

Computes approximately $1/\sqrt{x}$ for each double-precision floating-point value in a vector, with a relative error of less than $2^{-14}$.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| m128, m256, m512, xmm, ymm, zmm | xmm, ymm, zmm |

DO NOT support LOCK

This instruction is only available when the processor supports the AVX-512 foundation. It requires the execution environment to be in 64-bit mode or compatibility mode.

The instruction does not provide the full precision of a standard square root and reciprocal operation; it is an approximation. If higher precision is required, the result SHALL be used as an initial guess for a Newton-Raphson iteration. The instruction may trigger #P if the result is inexact. If the input is NaN, the result is the same NaN. If the input is $\pm 0$, the result is $\pm \infty$ and #Z is signaled. If the input is negative, the result is $qNaN$ and #I is signaled.
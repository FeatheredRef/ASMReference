> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# RSQRTSS

Computes the reciprocal square root of a scalar-precision floating-point value. It calculates $1 / \sqrt{x}$ with an accuracy of $\pm 1.5 \times 2^{-12}$ and a maximum relative error of $1.5 \times 2^{-12}$.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm (f32) | xmm (f32) |
| m32 (f32) | xmm (f32) |

DO NOT support LOCK

This instruction is available in 64-bit mode and compatibility mode. It requires the SSE instruction set.

The instruction does not perform a full-precision calculation; it is an approximation. To achieve higher precision, the result SHOULD be used as an initial guess for a Newton-Raphson iteration.

If the source operand is NaN, the result is the same NaN. If the source is $\pm 0$, the result is $\infty$ and #Z is raised. If the source is negative, the result is qNaN and #I is raised.

The following floating-point exceptions may be raised:
- #I: Invalid operation (negative input).
- #Z: Divide-by-zero (zero input).
- #P: Inexact result.
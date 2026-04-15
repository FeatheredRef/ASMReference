> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VRSQRT28SS

Computes the approximate reciprocal square root of a scalar single-precision floating-point value. The result is the reciprocal square root of the source operand with a relative error of less than $2^{-12}$.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm (f32) | xmm (f32) |
| m32 (f32) | xmm (f32) |

DO NOT support LOCK

This instruction is available only in 64-bit mode. It requires the SSE4.1 instruction set extension.

The precision of the result is limited; it provides an approximation and SHOULD NOT be used for high-precision calculations without subsequent Newton-Raphson iterations. If the source operand is $\pm0$, the result is $\pm\infty$ and #Z is raised. If the source operand is $\text{NaN}$, the result is a $\text{qNaN}$ and #I is raised. If the source operand is negative, the result is $\text{qNaN}$ and #I is raised. The instruction MAY trigger #P if the result is inexact.
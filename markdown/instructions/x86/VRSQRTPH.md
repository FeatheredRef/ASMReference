> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VRSQRTPH

Computes the approximate reciprocal square root of packed half-precision floating-point numbers.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm/ymm/zmm | xmm/ymm/zmm |

DO NOT support LOCK

This instruction is only available when AVX-512 is supported. It requires the use of the EVEX encoding scheme.

The instruction provides an approximation of $1/\sqrt{x}$. The precision of the result is limited; it is intended for use as a starting point for iterative refinement (e.g., using Newton-Raphson). It does not support memory operands as sources or destinations; operands MUST be registers.

The instruction produces a result with a relative error of less than $2^{-12}$. If the input is NaN, the result is NaN. If the input is $\pm 0$, the result is $\pm \infty$. If the input is negative, the result is NaN. These operations set the Floating-Point Status Register flags accordingly, including #P.
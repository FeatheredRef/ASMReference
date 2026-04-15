> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# RSQRTPS

Computes the approximate reciprocal square root of each single-precision floating-point element in a packed SIMD register.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |
| m128 | xmm |

DO NOT support LOCK

This instruction is available in both 32-bit and 64-bit modes. It requires the SSE extension to be supported by the processor.

The result is an approximation with a maximum relative error of $1.5 \times 2^{-12}$. To obtain higher precision, the result SHALL be used as an initial guess for a Newton-Raphson iteration.

If the input element is NaN, the result is the same NaN. If the input element is $\pm 0$, the result is $\infty$ and #Z is signaled. If the input element is negative, the result is qNaN and #I is signaled.

The instruction does not support memory-to-memory operations; the destination MUST be an xmm register.
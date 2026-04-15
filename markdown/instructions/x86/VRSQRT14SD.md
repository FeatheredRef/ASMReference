> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VRSQRT14SD

Computes the approximate reciprocal square root of a scalar double-precision floating-point value with a relative error of less than or equal to $2^{-14}$.

The following table describes the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| m8 | xmm |
| xmm | xmm |

DO NOT support LOCK

This instruction is available only when the AVX instruction set is supported. It operates on the lowest 64 bits of the xmm registers.

If the input is a NaN, the result is a qNaN. If the input is $\pm 0$, the result is $\pm \infty$. If the input is $-\infty$ or any negative number, the result is a qNaN.

The result is an approximation; therefore, users SHOULD NOT expect the precision of a full `SQRTSD` followed by a `DIVSD` operation. The instruction MAY trigger #P.
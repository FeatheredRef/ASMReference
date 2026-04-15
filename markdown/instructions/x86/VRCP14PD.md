> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VRCP14PD

Computes an approximate reciprocal of packed double-precision floating-point values. The instruction computes $1/x$ with a relative error of at most $2^{-14}$ for each double-precision element in the source.

The following table describes the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| xmm/ymm/zmm | xmm/ymm/zmm |
| m64/m128/m256/m512 | xmm/ymm/zmm |

DO NOT support LOCK

This instruction is available only in 64-bit mode or 32-bit mode when the AVX-512 foundation is supported. It requires the EVEX prefix for zmm registers.

The result is an approximation; therefore, it SHALL be used as an initial guess for iterative refinement (e.g., Newton-Raphson) if higher precision is required. The instruction does not trigger floating-point exceptions except for #P if the result is inexact. It does not support the lock prefix.
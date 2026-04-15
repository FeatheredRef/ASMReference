> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VRCP28SD

Computes the reciprocal approximation of a scalar double-precision floating-point value. The instruction calculates an approximation of $1/x$ with a relative error of $2^{-28}$.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm/ymm/zmm | xmm/ymm/zmm |

DO NOT support LOCK

This instruction is available only in 64-bit mode or compatibility mode. It requires the AVX instruction set to be enabled.

The result of the approximation is not exact; therefore, the #P exception may be raised. The instruction does not support memory operands for either source or destination; operands MUST be registers. If the input is a NaN, the result is a NaN. If the input is $\pm 0$, the result is $\pm \infty$.
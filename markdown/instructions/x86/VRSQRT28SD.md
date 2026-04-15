> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VRSQRT28SD

Computes the reciprocal square root of a scalar double-precision floating-point value with a relative error of less than or equal to $2^{-28}$.

The following table describes the supported sources and destinations.

| source | destination(s) |
| :--- | :--- |
| m8 | xmm |
| xmm | xmm |

DO NOT support LOCK

This instruction is available only when the processor is operating in 64-bit mode or compatibility mode. It requires the AVX instruction set to be enabled.

The destination register is overwritten by the result. If the input is a signaling NaN, a #I exception SHALL be generated. If the input is $\pm 0$, the result is $\infty$ and a #Z exception SHALL be generated. If the input is $\pm \infty$, the result is $+0$ and a #P exception SHALL be generated. The precision of the result is limited to the specified 28-bit accuracy, which MAY result in a #P exception.
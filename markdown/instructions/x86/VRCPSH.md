> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VRCPSH

Computes the reciprocal of the sum of two floating-point values. Specifically, it calculates $1 / (a + b)$ and stores the result in the destination operand.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| xmm reg | xmm reg |
| m32 | xmm reg |
| m64 | xmm reg |
| m128 | xmm reg |

DO NOT support LOCK

This instruction is available only in 32-bit mode and 64-bit mode. It requires the SSE3 extension to be supported by the processor.

The operation is performed using the precision defined by the MXCSR register. The result is an approximation and does not guarantee the highest precision; it is intended for use in scenarios where a fast reciprocal estimate is sufficient. Precision exceptions (#P) may be raised depending on the rounding mode and the result's proximity to the exact value.
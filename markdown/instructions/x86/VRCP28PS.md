> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VRCP28PS

Computes an approximation of the reciprocal of the four single-precision floating-point values in the source operand and stores the results in the destination operand. The approximation is accurate to within 2^-28.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |
| m128 | xmm |

DO NOT support LOCK

This instruction is only available when the CPU supports the SSE4.1 instruction set. It operates on XMM registers and is supported in both 32-bit and 64-bit modes.

The result of VRCP28PS is an approximation and NOT a precise reciprocal. To obtain a higher precision result, it SHALL be used in conjunction with the VRCP14PS instruction or a Newton-Raphson iteration. If the input is $\pm 0$, the result is $\pm \infty$; if the input is $\pm \infty$, the result is $\pm 0$; if the input is NaN, the result is NaN. This instruction does NOT trigger floating-point exceptions such as #Z, #D, #O, #U, or #P.
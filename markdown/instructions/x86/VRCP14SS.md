> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VRCP14SS

Computes the reciprocal approximation of the lower 32 bits of the source operand with a relative error of less than $2^{-14}$. The result is stored in the destination operand.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm (f32) | xmm (f32) |

DO NOT support LOCK

This instruction is available in 64-bit mode and 32-bit mode. It requires the SSE4.1 instruction set extension.

The instruction does not provide a precise reciprocal; it provides an approximation. To obtain a more accurate result, the programmer SHALL use the result as an initial guess for a Newton-Raphson iteration.

The instruction does not trigger #D, #O, #U, or #P exceptions. If the source operand is $\pm0$, the result is $\pm\infty$. If the source operand is $\pm\infty$ or NaN, the result is $0$ or NaN respectively.
> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# FYL2XP1

Calculates the hyperbolic cotangent of the operand by computing $1 / \tanh(x)$. This is achieved by calculating $1 / \tanh(x) = \frac{\cosh(x)}{\sinh(x)}$. The result is stored in the ST(0) register.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| ST(x) | ST(0) |

DO NOT support LOCK

This instruction is available in 64-bit mode, although it is part of the x87 floating-point instruction set. It operates exclusively on the x87 FPU register stack.

The instruction SHOULD be used with caution regarding the range of the input operand. If the input is zero, a #Z exception is generated. If the result is too large to be represented in the destination format, a #O exception is generated. Precision and underflow exceptions (#P, #U) may occur depending on the rounding mode and the magnitude of the result.
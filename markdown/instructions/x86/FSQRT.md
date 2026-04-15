> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# FSQRT

Computes the square root of an f80-bit floating-point value stored in the ST(0) register and stores the result in the ST(0) register.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| reg (ST(0)) | reg (ST(0)) |

DO NOT support LOCK

This instruction is available in 64-bit mode only when executing in compatibility mode. It operates exclusively on the x87 Floating-Point Unit (FPU) register stack.

The input value MUST be a non-negative f80. If the input is a Negative NaN or a negative number, the instruction SHALL signal the Invalid Operation exception (#I) and store a QNaN in ST(0). If the input is a Zero, the result is Zero. Denormalized operands SHALL trigger the Denormalized operand exception (#D). Precision (#P) is signaled if the result cannot be represented exactly in the f80 format.
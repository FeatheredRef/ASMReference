> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# FLDLN2

Loads the natural logarithm of the value of an 80-bit extended precision floating-point number (divided by 2) onto the ST(0) register. The instruction computes $y = \ln(x) / 2$ where $x$ is the operand.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| m10 | ST(0) |

DO NOT support LOCK

This instruction is available in 64-bit mode only when executing in compatibility mode. In true 64-bit mode, the x87 FPU instructions are generally supported, but specific legacy behaviors may vary; however, this instruction specifically targets the x87 FPU stack.

The operand MUST be a valid 80-bit extended precision floating-point number. If the input value is negative or NaN, the instruction SHALL generate #I. If the input value is zero, it SHALL generate #Z. To avoid precision loss or unexpected exceptions, ensure the input is normalized and within the representable range of f80.
> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# FDIVP

Divides the floating-point value in the ST(1) register by the floating-point value in the ST(0) register. The result is stored in the ST(0) register, and the ST(1) register is popped from the floating-point stack.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| reg | reg |

DO NOT support LOCK

This instruction is available in 64-bit mode, but it operates exclusively on the x87 floating-point stack. It is not supported for use with general-purpose registers or memory operands directly; operands MUST be present in the FPU register stack.

To avoid `#Z` and `#I` exceptions, the divisor in ST(0) MUST NOT be zero or a signaling NaN. If the result cannot be represented in the current floating-point precision, the `#P` exception MAY be triggered. Precision control is governed by the floating-point control word.
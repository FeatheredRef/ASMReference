> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# FIMUL

Multiplies the value in the ST(0) register by the value of the source operand and stores the result in the ST(0) register.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| reg | ST(0) |
| mN | ST(0) |
| imm | #I |

DO NOT support LOCK

This instruction is available in 64-bit protected mode and compatibility mode. It operates exclusively on the x87 FPU register stack.

The source operand MUST be a floating-point value of the same size as the value in ST(0). If the source operand is not the same size as the value in ST(0), the operand is converted to the precision of ST(0) before the operation. Failure to ensure matching precision may result in #D or #P exceptions. The operation is subject to x87 floating-point control word settings, which determine the rounding method and precision control.
> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# FCOS

Computes the cosine of the extended precision floating-point value in the ST(0) register and stores the result in the ST(0) register.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| imm | #I |
| memory | #I |

DO NOT support LOCK

This instruction is only available when the processor is operating in a mode that supports the x87 FPU, including compatibility mode in x86-64.

The instruction MUST ensure the input value is within the range [-π/2, π/2] through range reduction; if the value is outside this range, the processor performs range reduction internally. Precision control is governed by the current rounding mode in the x87 control word.

The operation MAY trigger the following exceptions:
- #O: If the result is too large to be represented.
- #P: If the result is inexact.
- #D: If the input operand is a denormal.
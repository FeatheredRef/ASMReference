> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# FSUBR

Subtracts the value of the ST(0) floating-point stack top from a specified operand and stores the result in ST(0).

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg | ST(0) |
| m64 | ST(0) |
| m32 | ST(0) |
| m16 | ST(0) |

DO NOT support LOCK

This instruction is available in 64-bit mode, but it operates exclusively on the x87 FPU register stack. It is subject to the current x87 floating-point control word settings.

The operation performs the calculation `ST(0) = operand - ST(0)`. Users MUST be aware that this differs from `FSUB`, which performs `ST(0) = ST(0) - operand`. Failure to distinguish between these two results in a sign inversion of the output.

The instruction may trigger the following exceptions:
- #P if the result is inexact.
- #O if the result overflows.
- #U if the result underflows.
- #D if an operand is denormal.
- #Z if the operation results in division by zero (not applicable to subtraction unless dealing with specific NaN cases).
- #I if any operand is a signaling NaN.
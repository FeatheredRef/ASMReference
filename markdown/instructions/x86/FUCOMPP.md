> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# FUCOMPP

Compares the value in an ST(0) floating-point stack element with a floating-point value from a specified source and sets the floating-point status word flags.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| reg | ST(0) |
| m8 | ST(0) |
| m16 | ST(0) |
| m32 | ST(0) |
| m64 | ST(0) |
| m128 | ST(0) |

DO NOT support LOCK

This instruction is available in x86-64 mode only via compatibility mode. It is not supported in 64-bit mode.

The operation is performed based on the current precision control set in the floating-point control word. If the operand is a memory reference, the size of the memory operand determines the precision of the comparison.

The instruction MAY trigger floating-point exceptions: #I if an operand is a NaN, #D if an operand is denormal, #O if the result cannot be represented in the current precision, #U if an underflow occurs, and #P if the result is inexact. The result of the comparison is stored in the FPU status word (C0, C1, C2, C3 flags).
> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# FDIVR

Divides the value in the ST(0) register by a value from a source operand, then reverses the order of the operands by popping the source operand from the stack (if the source is ST(i)) or by effectively treating the operation as a reverse division. In the case of `FDIVR st(i), st(j)`, it computes `ST(j) / ST(i)` and stores the result in `ST(j)`.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| memory | reg |

DO NOT support LOCK

This instruction is only available in x86-64 when the processor is operating in compatibility mode. It is not supported in 64-bit mode.

The instruction may trigger the following floating-point exceptions: #Z if the divisor is zero, #O if the result is too large to be represented, #U if the result is too small to be represented, #D if a denormal operand is encountered, and #P if the result is inexact. These exceptions are subject to the mask bits in the floating-point control word.
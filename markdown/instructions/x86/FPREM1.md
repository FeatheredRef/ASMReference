> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# FPREM1

Computes the remainder of a division operation. It divides the value in the ST(0) register by the value in a specified register or memory location, and stores the remainder in ST(0).

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg | ST(0) |
| m64 | ST(0) |

DO NOT support LOCK

This instruction is available in 64-bit mode and compatibility mode. It operates exclusively on the x87 FPU register stack.

The instruction MUST be used in conjunction with `FPREM` to complete a full division if the quotient exceeds the capacity of the 64-bit integer representation. `FPREM1` computes only the first partial remainder. If the quotient is larger than $2^{63}$, the result in ST(0) will be incorrect unless `FPREM` is called repeatedly.

The instruction MAY trigger the following exceptions:
- #Z: If the divisor is zero.
- #O: If the result cannot be represented.
- #D: If an operand is denormal.
- #P: If the result is inexact.
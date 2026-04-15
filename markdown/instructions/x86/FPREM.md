> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# FPREM

Computes the remainder of the division of the ST(0) register by a specified operand. The result is stored in ST(0) and the quotient is stored in ST(1).

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg | ST(0), ST(1) |
| m64 | ST(0), ST(1) |
| m32 | ST(0), ST(1) |
| m16 | ST(0), ST(1) |

DO NOT support LOCK

This instruction is available in 64-bit mode and compatibility mode. It operates on the x87 Floating-Point Unit (FPU) register stack.

The operation MUST ensure that the divisor is not zero to avoid a #Z exception. If the quotient is too large to be represented in the destination register, a #O exception SHALL occur. Users SHOULD be aware that FPREM is an iterative instruction; it does not necessarily compute the full remainder in a single execution. It is RECOMMENDED to wrap FPREM in a loop, checking the Status Word (specifically the Control Word's precision and the FPU's status flags) to verify if the remainder has been fully computed, as the instruction may require multiple calls to reach the final result. Failure to do so MAY result in an imprecise remainder.
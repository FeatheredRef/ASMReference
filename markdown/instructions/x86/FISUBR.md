> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# FISUBR

Subtracts the ST(0) floating-point value from a specified floating-point value and stores the result in ST(0).

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| reg | ST(0) |
| memory | ST(0) |
| imm | #I |

DO NOT support LOCK

This instruction is available in 64-bit mode, but it operates exclusively on the x87 Floating-Point Unit (FPU) stack. It does not utilize XMM or YMM registers.

To avoid precision loss or unexpected exceptions, the user SHALL ensure that the FPU control word is correctly configured for the desired rounding mode. If the result cannot be represented in the destination format, the FPU SHALL signal #P. If the result exceeds the representable range of the floating-point format, the instruction SHALL signal #O.
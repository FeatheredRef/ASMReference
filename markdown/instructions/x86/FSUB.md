> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# FSUB

Subtracts the ST(0) floating-point value from a source operand and stores the result in ST(0).

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| reg (ST(i)) | ST(0) |
| mN | ST(0) |
| imm | #I |

DO NOT support LOCK

This instruction is part of the x87 floating-point unit. It is supported in 64-bit mode but operates on the x87 register stack rather than XMM/YMM/ZMM registers.

The instruction may trigger floating-point exceptions including #P, #U, #O, #D, and #Z depending on the rounding mode and the values of the operands.

When using the memory-operand version, the size of the memory operand (mN) MUST match the current precision specified in the x87 control word. If the memory operand size does not match the precision, it MAY result in an unexpected value or a precision exception.
> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# KORTESTQ

Performs a bitwise logical AND between the first source operand and the second source operand, then sets the ZF, SF, and PF flags based on the result. The destination operand is not modified.

The following table covers the supported sources and destinations:

| source | destination(s) |
| :--- | :--- |
| reg | #I |
| m8 | #I |
| imm | #I |

DO NOT support LOCK

This instruction is only available in 64-bit mode. It cannot be used in compatibility mode.

The instruction does not modify the destination operand; however, if a memory operand is specified as a destination in the opcode encoding, it is ignored. Users SHOULD ensure that the operands are 64-bit (qword) to avoid unexpected behavior with smaller register aliases. Since this is a non-destructive test, it is specifically designed to preserve the values of the source operands while updating the EFLAGS register.
> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# CDQE

Sign-extends a dword operand to a qword destination.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg (r32) | reg (r64) |
| imm | #I |
| m4 | #I |

DO NOT support LOCK

This instruction is only available in 64-bit mode. It is NOT supported in compatibility mode.

The destination register MUST be the 64-bit extension of the source r32 register (e.g., EAX to RAX). Attempting to use this instruction with mismatched registers is not supported by the opcode encoding.
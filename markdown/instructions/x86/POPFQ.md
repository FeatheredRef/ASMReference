> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# POPFQ

Pops a qword from the top of the stack and loads it into the RFLAGS register.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| m8 | #I |
| m16 | #I |
| m32 | #I |
| m64 | RFLAGS |

DO NOT support LOCK

This instruction is available in 64-bit mode. In 64-bit mode, the operand size is fixed to qword.

The RFLAGS register contains reserved bits that are not modified by this instruction. Attempting to modify reserved bits via POPFQ will have no effect on those specific bits. Only the bits defined in the RFLAGS architecture are updated.

When using POPFQ, the stack pointer (RSP) is incremented by 8 bytes after the value is popped. Failure to ensure the stack is properly aligned or contains valid data may lead to undefined processor states or General Protection faults if the stack pointer crosses a segment boundary.
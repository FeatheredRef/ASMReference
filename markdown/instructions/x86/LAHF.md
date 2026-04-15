> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# LAHF

Copies the lower 8 bits of the EFLAGS register to the specified register.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| EFLAGS (low 8 bits) | r8 |
| #I | m8 |
| #I | imm |

DO NOT support LOCK

This instruction is available only in 32-bit mode and compatibility mode. It SHALL NOT be used in 64-bit mode, where it is considered an invalid opcode.

The instruction only affects the destination register and does not modify any flags in the EFLAGS register. Since this instruction is not supported in 64-bit mode, developers MUST ensure the processor is executing in a 32-bit environment or compatibility mode to avoid illegal instruction exceptions.
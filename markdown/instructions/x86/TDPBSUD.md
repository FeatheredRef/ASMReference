> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# TDPBSUD

This instruction updates the Table Descriptor Base Address and the Table Descriptor Size in the specified descriptor table register.

The following table describes the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| imm | #I |
| mN | #I |

DO NOT support LOCK

This instruction is ONLY available in 64-bit mode. It is NOT supported in compatibility mode or 32-bit protected mode.

The instruction MUST be executed with CPL 0. If the current privilege level is not 0, a general protection fault (#GP) SHALL be generated. The destination register MUST be one of the supported descriptor table registers (GDTR, IDTR, PDTR, or LDTR). Attempts to use an unsupported register as a destination SHALL result in an invalid opcode exception.
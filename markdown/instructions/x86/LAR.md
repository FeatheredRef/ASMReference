> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# LAR

Loads the selector from the Local Descriptor Table register (LDTR) into the specified register.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| LDTR | r16/r32 |
| #I | mN |
| #I | imm |

DO NOT support LOCK

This instruction is NOT supported in 64-bit mode. It is only available in compatibility mode and legacy 32-bit/16-bit modes.

The destination register MUST be a general-purpose register. If the destination is a 32-bit register, the upper 32 bits of the 64-bit register (in compatibility mode) are set to zero. Failure to use this instruction within a supported mode SHALL result in an invalid opcode exception.
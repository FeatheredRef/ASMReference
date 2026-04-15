> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# WRUSSD

Writes a 64-bit value from a register to the User State Segment Descriptor (USSD) in the processor's internal state.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| r64 | USSD |
| imm | #I |
| m8 | #I |

DO NOT support LOCK

This instruction is only available when the processor is operating in 64-bit mode. It is not supported in compatibility mode or 32-bit protected mode.

Failure to provide a valid r64 register as the source will result in an invalid opcode exception. The instruction does not affect the EFLAGS register.
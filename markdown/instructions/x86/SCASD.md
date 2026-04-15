> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# SCASD

Subtracts the source operand from the destination operand and sets the carry flag if the result is negative, then clears the carry flag otherwise.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| imm | reg |
| #I | mN |

DO NOT support LOCK

This instruction is only available in compatibility mode. It operates on the `eax` register as the implicit destination.

The instruction is specifically designed for legacy 32-bit calculations within a 64-bit environment; attempting to use this instruction in 64-bit mode SHALL result in an invalid opcode exception. Ensure the operand size override is not used to target 64-bit registers.
> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# CWDE

Convert Word to Doubleword Extension. This instruction extends the sign of a 16-bit word value to a 32-bit doubleword. It takes the value from a source operand and sign-extends it into the destination.

The following table covers the supported source and destination operands:

| source | destination(s) |
| :--- | :--- |
| reg (r16) | reg (r32) |
| m2 | reg (r32) |
| #I | imm |
| #I | m4 |

DO NOT support LOCK

This instruction is available in 64-bit mode and compatibility mode. It requires the source to be a 16-bit value and the destination to be a 32-bit general-purpose register.

When using this instruction, the user MUST ensure that the destination register is intended to be overwritten, as the operation replaces the lower 32 bits of the destination register. Because this is a 32-bit operation in x86-64, the upper 32 bits of the destination r64 register are zeroed by the hardware.
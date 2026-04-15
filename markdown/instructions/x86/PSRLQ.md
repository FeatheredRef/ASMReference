> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PSRLQ

Shifts the destination operand to the right by the count specified in the source operand. The vacated bit positions are filled with zeros.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| imm | reg |
| #I | m8 |
| #I | m16 |
| #I | m32 |
| #I | m64 |

DO NOT support LOCK

This instruction is available only in 64-bit mode. In compatibility mode, this instruction is not supported and will trigger an invalid opcode exception.

The shift count is masked to 6 bits (0 to 63). If the source operand is a register, only the low-order 6 bits of the register are used to determine the shift count. Providing a value greater than 63 will not shift the operand by that value, but by the result of the value modulo 64.
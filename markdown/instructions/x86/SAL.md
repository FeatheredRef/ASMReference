> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# SAL

Shifts the destination operand to the left by the count specified in the source operand. The most significant bit of the destination operand is shifted into the Carry Flag (CF), and zeros are shifted into the least significant bit.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| imm | reg |
| reg | mN |
| imm | mN |

DO NOT support LOCK

The instruction is available in 16-bit, 32-bit, and 64-bit operand sizes. In 64-bit mode, if the destination is an r64 register, the shift count is masked to 6 bits (0-63).

The SAL instruction is functionally identical to SHL.

When using an imm as the source, the immediate SHALL be an unsigned 8-bit value. If a reg is used as the source, only the low-order 6 bits of the register are used for the shift count when the destination is a qword. Using a shift count equal to or greater than the operand size results in a shift of 0 bits if the architecture implements masking, or undefined behavior if not.
> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# SAR

Shifts the destination operand to the right by the count specified in the source operand. The sign bit of the destination operand is replicated into the most significant bit, preserving the sign of the iN value (arithmetic shift).

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| imm | rN |
| reg | rN |
| imm | mN |
| reg | mN |

DO NOT support LOCK

The shift count is masked to 5 bits if the operand size is 32 bits, or 6 bits if the operand size is 64 bits. This means the effective shift count is `source AND 31` for dword and `source AND 63` for qword.

The instruction is available in 16-bit, 32-bit, and 64-bit operand sizes. In 64-bit mode, when using a register as the shift count, the register must be clamped to the appropriate bit width based on the destination operand size.

The Carry Flag (CF) contains the value of the last bit shifted out. If the shift count is 0, the CF remains unchanged. If the shift count is greater than or equal to the operand size, the CF is undefined.

The Zero Flag (ZF) and Sign Flag (SF) are updated based on the result. Other status flags are affected according to Intel SDM specifications for shift operations.
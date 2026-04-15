> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# ROL

Rotates the bits of the source operand to the left by the count specified in the second operand. Bits that are shifted out of the most significant bit (MSB) position are wrapped around and shifted into the least significant bit (LSB) position.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| mN | reg |
| reg | mN |
| imm | #I |

DO NOT support LOCK

The instruction is supported in 64-bit mode, compatibility mode, and 32-bit mode. When operating on qword registers in 64-bit mode, the rotate count is masked to 6 bits (modulo 64). In 32-bit mode, the rotate count is masked to 5 bits (modulo 32).

The shift count must be specified either as an immediate value or in the CL register. If the shift count is provided via a register other than CL, the operation is invalid. When using an immediate value, it SHALL be an unsigned constant. The Carry Flag (CF) is updated: the value of the last bit rotated from the MSB into the LSB is stored in CF.
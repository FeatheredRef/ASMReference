> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PSLLQ

Shifts the 64-bit quadword value in the destination operand to the left by the number of bits specified by the source operand. Vacant bit positions are filled with zeros.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| imm | reg |
| #I | m8 |
| #I | m16 |
| #I | m32 |
| #I | m64 |

DO NOT support LOCK

This instruction is only available in 64-bit mode. It is NOT supported in compatibility mode.

The shift count is masked to 6 bits (0-63) for the `PSLLQ` instruction. If the source operand is a register, only the low-order 6 bits of the register are used; any bits beyond the 6th bit are ignored. This prevents undefined behavior when a value greater than 63 is provided.
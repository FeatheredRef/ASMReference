> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# KSHIFTLQ

Shifts the bits of a 64-bit quad word to the left by the count specified in the source operand. Bits shifted out of the most significant bit are discarded, and zeros are shifted into the least significant bit.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| imm | reg |
| #I | m8 |
| #I | m16 |
| #I | m32 |
| #I | m64 |

DO NOT support LOCK

This instruction is ONLY available in 64-bit mode. It SHALL NOT be used in compatibility mode or 32-bit protected mode.

The shift count is masked to 6 bits (modulo 64). If the source is a reg, only the low 6 bits of the register are used. This prevents shifts of 64 bits or more, which would result in the original value being shifted entirely out of the register.
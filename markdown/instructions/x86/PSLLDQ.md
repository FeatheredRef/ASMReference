> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PSLLDQ

Shifts a 64-bit quad word to the left by a specified immediate number of bits. Bits shifted out of the most significant position are discarded, and zeros are shifted into the least significant positions.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| imm | r64 |
| imm | m8 |

DO NOT support LOCK

This instruction is available in 64-bit mode. It is NOT supported in compatibility mode.

The immediate operand SHALL be a constant value between 0 and 63. If the immediate value is outside this range, the instruction is invalid. Unlike the general-purpose shift instructions (SHL), PSLLDQ does not use the CL register; it requires an immediate value. The destination register or memory location MUST be 64 bits wide.
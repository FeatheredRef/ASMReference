> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VPSRLVW

Shifts each 32-bit doubleword lane of a packed YMM or ZMM register to the right by a count specified in a register or immediate. The shift is logical, meaning zero bits are shifted into the most significant bit position of each lane.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm/ymm/zmm | xmm/ymm/zmm |
| imm | xmm/ymm/zmm |

DO NOT support LOCK

This instruction is only available in 64-bit mode. It requires the AVX-512F instruction set extension.

The shift count is masked to 5 bits (u5) when provided via a register, meaning only the lower 5 bits of the source register are used to determine the shift distance. If the immediate value is used, it MUST be within the range 0-31. Failure to provide a valid immediate in the opcode encoding will result in an invalid instruction.
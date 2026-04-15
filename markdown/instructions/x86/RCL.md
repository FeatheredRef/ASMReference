> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# RCL

Rotates the destination operand to the left by the count specified in the source operand. The Carry Flag (CF) is rotated into the least significant bit of the destination, and the most significant bit of the destination is rotated into CF.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| reg | mN |
| imm | reg |
| imm | mN |

DO NOT support LOCK

The instruction is available in 64-bit mode, 32-bit mode, and 16-bit mode. In 64-bit mode, if a 64-bit operand size is used, the shift count is masked to 6 bits (0-63).

When using an imm as the source, the immediate MUST be a byte. If the count is specified in a reg, the count is masked to 6 bits for 32-bit or 64-bit operands, and 5 bits for 16-bit operands. Failure to account for this masking may result in unexpected rotation counts.
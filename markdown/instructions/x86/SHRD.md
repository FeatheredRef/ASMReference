> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# SHRD

Shifts the destination operand to the right by the count specified in the source operand. The bits shifted out of the destination are shifted into the source operand (if it is a register), and the bits shifted out of the source are shifted into the destination.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| reg | mN |
| #I | imm |

DO NOT support LOCK

The instruction is available in 64-bit mode and compatibility mode. When operating on 64-bit operands, the shift count is masked to 6 bits (0-63) if the count is specified in a register.

If the destination is a register, the source must be a register to allow the bidirectional shift of bits. If the destination is a memory location (mN), the source must be a register, and the source register remains unchanged after the operation.
> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# KORB

Performs a bitwise logical XOR operation between the source operand and the destination operand, storing the result in the destination. The operation is performed on the least significant 8 bits of the operands.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| r8 | r8 |
| m1 | r8 |
| imm | r8 |
| r8 | m1 |
| m1 | m1 |
| imm | m1 |

DO NOT support LOCK

This instruction is available in 64-bit mode and compatibility mode. It operates specifically on 8-bit registers or memory locations.

The destination operand MUST be an 8-bit register or memory location to avoid size mismatch. Using a larger register as a destination for `KORB` will result in an assembly error or unexpected behavior if manually encoded.
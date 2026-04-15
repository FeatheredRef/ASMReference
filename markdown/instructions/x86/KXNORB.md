> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# KXNORB

Performs a bitwise XNOR operation on the source and destination operands. The result is the logical NOT of the XOR of the operands, stored in the destination.

The following table covers the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| r8 | r8 |
| m8 | r8 |
| imm | r8 |
| r8 | m8 |
| imm | m8 |

DO NOT support LOCK

This instruction is only available in 64-bit mode. It SHALL NOT be used in compatibility mode or 32-bit mode.

The destination operand MUST be an 8-bit register or memory location to avoid undefined behavior or size mismatches. Ensure that the memory operand is aligned to a 1-byte boundary to prevent performance penalties, although it is functionally required by the ISA.
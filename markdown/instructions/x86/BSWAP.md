> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# BSWAP

Reverses the byte order of a dword or qword.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| m4/m8 | reg |
| #I | m4/m8 |
| imm | #I |

DO NOT support LOCK

This instruction is available in 32-bit and 64-bit modes. In 64-bit mode, the instruction can operate on either a dword or a qword. In compatibility mode, the behavior is consistent with the 32-bit operation for dword operands.

When utilizing `BSWAP` on a memory operand, the result is loaded into a register; it cannot store the result directly back to memory. To swap bytes in memory, the value MUST be loaded into a register, processed with `BSWAP`, and then stored back to the memory location.
> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# BLSR

Resets the lowest set bit of the destination operand to zero. If the destination operand is 0, the destination operand remains 0 and the carry flag is cleared. Otherwise, the lowest set bit is cleared and the carry flag is set.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| #I | reg/mN |

DO NOT support LOCK

This instruction is available in 64-bit mode and 32-bit mode. It is NOT available in 16-bit mode.

The destination operand MUST be a register or a memory location of size 8, 16, or 32 bits. If the destination operand is a memory location, the result is written back to that memory location.

To avoid incorrect flag evaluations, be aware that the CF is set based on whether the input was non-zero. If the input is already 0, CF is cleared, which is the only way to distinguish a zero input from a non-zero input that had its only set bit cleared.
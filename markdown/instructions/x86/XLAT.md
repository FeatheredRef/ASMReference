> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# XLAT

Replaces the byte at `AL` with a byte from a lookup table in memory. The table starts at the address contained in `BX`. The byte at `AL` is used as an unsigned index into this table.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| #I | mN |
| #I | imm |

DO NOT support LOCK

XLAT is only supported in 16-bit mode or 32-bit compatibility mode. It is NOT supported in 64-bit mode. Attempting to execute this instruction in 64-bit mode SHALL result in an invalid opcode exception.

The instruction implicitly uses `AL` as both the index and the destination, and `BX` as the base address. Because it relies on these specific 8-bit and 16-bit registers, it cannot be used with general-purpose registers of larger sizes (e.g., `RAX` or `RBX`) beyond the implicit architectural mapping. Failure to ensure `BX` contains the correct base address before execution will result in a memory access violation or the retrieval of incorrect data.
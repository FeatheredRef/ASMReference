> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# MOVMSKPD

Sets each bit of the destination general-purpose register to 1 if the corresponding double-precision floating-point element of the source XMM register has its sign bit set, and to 0 otherwise.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm | r32 |
| #I | mN |
| #I | imm |

DO NOT support LOCK

This instruction is available in both 32-bit and 64-bit modes. It requires the source to be an XMM register; memory-to-register movement for the mask generation is NOT supported.

The destination register is cleared before the mask is written, meaning bits 4 through 31 of the r32 register are set to 0. Failure to account for the clearing of the upper bits of the destination register MAY lead to incorrect logic if the register was intended to be used as a bitmask for other purposes.
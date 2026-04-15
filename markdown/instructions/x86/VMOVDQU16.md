> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VMOVDQU16

Moves 128 bits of data from the source to the destination. The memory operand is not required to be aligned on a 16-byte boundary.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm reg | xmm reg |
| m16 | xmm reg |
| xmm reg | m16 |

DO NOT support LOCK

This instruction SHALL only be executed in 64-bit mode or 32-bit mode. It REQUIRES the AVX support feature. The instruction is encoded using the VEX prefix.

The destination memory operand m16 SHALL be accessed using the current memory segmentation and paging attributes. If the memory operand crosses a page boundary, a general-protection exception may occur if the access violates memory protection. Using unaligned memory access on older microarchitectures MAY result in a performance penalty compared to aligned moves.
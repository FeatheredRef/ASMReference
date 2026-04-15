> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VMOVDQU32

Moves 32 bytes of unaligned data from the source to the destination.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| xmm/ymm/zmm reg | xmm/ymm/zmm reg |
| m32 | xmm/ymm/zmm reg |
| xmm/ymm/zmm reg | m32 |

DO NOT support LOCK

This instruction SHALL be used in 64-bit mode or compatibility mode. It REQUIRES AVX support.

To avoid alignment exceptions or performance degradation, ensure that the memory operand m32 is accessed via an unaligned move if the address is not 32-byte aligned. Using this instruction instead of `VMOVDQA32` prevents a General Protection exception (#GP) when the memory address is not aligned to a 32-byte boundary.
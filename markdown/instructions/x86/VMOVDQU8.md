> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VMOVDQU8

Moves 64 bytes of data from the source to the destination. The operation is performed as an unaligned move of eight 64-bit elements.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| zmm reg | zmm reg |
| m64 | zmm reg |
| zmm reg | m64 |

DO NOT support LOCK

This instruction is ONLY available when AVX-512 is supported. It requires the EVEX prefix and is not available in compatibility mode.

The destination memory address SHALL NOT be aligned to a 64-byte boundary to avoid performance penalties, although the instruction explicitly supports unaligned access. If the memory operand spans a page boundary, a page fault MAY occur.
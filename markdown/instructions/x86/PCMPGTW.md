> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PCMPGTW

Compares two packed signed word integers in the source operands. For each pair of words, if the first word is greater than the second word, the corresponding word in the destination is set to all ones (0xFFFF); otherwise, it is set to all zeros (0x0000).

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |
| m16 | xmm |

DO NOT support LOCK

This instruction is available in 64-bit mode and 32-bit mode. It requires the SSE4.1 instruction set extension.

The destination register must not be the same as the memory source operand if the instruction is implemented as a non-destructive operation in older microarchitectures. When using a memory operand, the memory region MUST be 16-byte aligned to avoid performance penalties or general protection faults depending on the alignment check settings.
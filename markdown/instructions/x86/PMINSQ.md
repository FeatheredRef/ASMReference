> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PMINSQ

Computes the minimum of two packed signed qword integers. For each corresponding pair of elements in the source operands, the smaller value is selected and stored in the destination.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm/ymm/zmm register | xmm/ymm/zmm register |
| m8/m16/m32/m64 | xmm/ymm/zmm register |

DO NOT support LOCK

This instruction is available only in 64-bit mode. It is NOT supported in compatibility mode.

The instruction requires the SSE4.1 instruction set extension or higher. Using this instruction on a processor that does not support SSE4.1 SHALL result in an invalid opcode exception.

When using the three-operand form (VPMINSQ), the destination register can be different from the source registers, preventing the overwriting of source data. If using the two-operand form, the first operand SHALL act as both a source and the destination.
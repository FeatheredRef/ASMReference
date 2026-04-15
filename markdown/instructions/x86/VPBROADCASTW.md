> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VPBROADCASTW

Broadcasts the 16-bit word from the source operand to all 16-bit elements of the destination vector register.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| m2 | zmm |
| xmm | zmm |

DO NOT support LOCK

The instruction is only available in 64-bit mode. It requires the AVX-512 foundation extensions to be enabled.

The memory operand MUST be aligned to the natural boundary of the data size to avoid performance penalties, although the instruction itself does not mandate alignment for correctness. If the destination is a zmm register, the upper bits of the register are overwritten; this instruction does NOT support a masked version for the destination register in the base form.
> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VPERMQ

Permutes six quadwords from the source operand according to the indices specified in the immediate operand. The instruction selects 6 of the 8 available 64-bit elements from the source and places them into the destination.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| zmm reg | zmm reg |
| zmm m64 | zmm reg |

DO NOT support LOCK

This instruction REQUIRES the AVX-512DQ instruction set. It is only available in 64-bit mode.

The immediate operand MUST be a valid index (0-7) for each of the 6 destination quadwords; indices outside this range SHALL result in the corresponding destination element being zeroed. Since this instruction uses a 512-bit register but only operates on 6 quadwords, the upper 2 quadwords (elements 6 and 7) of the destination zmm register are zeroed.
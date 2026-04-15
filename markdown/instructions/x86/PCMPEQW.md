> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PCMPEQW

Compares two 128-bit XMM operands as packed words. For each pair of corresponding words, if the words are equal, the corresponding 16 bits of the destination are set to 1; otherwise, they are set to 0.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm reg | xmm reg |
| m16 | xmm reg |

DO NOT support LOCK

This instruction is available in both 32-bit and 64-bit mode. It requires the SSE2 instruction set extension.

The destination register is overwritten by the result of the comparison. If the destination is also a source operand, the original values are consumed before the result is written. Memory operands SHALL be aligned to 16 bytes if using the aligned move variants; otherwise, unaligned memory access support depends on the CPU's alignment check and SSE settings.
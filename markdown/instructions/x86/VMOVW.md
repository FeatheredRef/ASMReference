> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VMOVW

Moves a packed word value from the source to the destination.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| xmm reg | xmm reg |
| m2 | xmm reg |
| xmm reg | m2 |

DO NOT support LOCK

This instruction is available in 64-bit mode and compatibility mode. It requires the SSE2 instruction set extension.

The instruction operates on the lowest 16 bits of the XMM registers. The upper bits of the destination register remain unchanged. When using a memory operand, the memory access SHALL be aligned to a 2-byte boundary to avoid performance penalties, although unaligned access is supported.
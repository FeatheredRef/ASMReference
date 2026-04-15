> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VEXTRACTF128

Extracts a 128-bit lane from a 256-bit XMM/YMM register and stores it in the destination XMM register. The specific lane is determined by the immediate operand.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm/ymm reg | xmm reg |

DO NOT support LOCK

This instruction SHALL only be executed when the AVX instruction set is supported. It is available in both 64-bit mode and compatibility mode.

The immediate operand MUST be either 0 or 1. If an immediate other than 0 or 1 is provided, the instruction SHALL encode as invalid or result in an undefined operation depending on the assembler. Using an immediate of 0 extracts the low 128 bits; an immediate of 1 extracts the high 128 bits. Mixing YMM and XMM registers without proper state transitions MAY lead to performance penalties due to AVX-SSE transitions.
> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VEXTRACTI32x8

Extracts eight 32-bit elements from a 256-bit vector register and stores the result in a general-purpose register.

The following table specifies the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| xmm/ymm reg | r64 |
| #I | m64 |

DO NOT support LOCK

This instruction is available only in 64-bit mode. It requires the AVX instruction set extension to be enabled.

The index specified by the immediate operand MUST be within the range 0 to 1. If the index is out of range, it SHALL trigger a General Protection (#GP) exception. Since the instruction targets a 64-bit general-purpose register, the extracted 32-bit elements are concatenated; however, because the source is 32-bit elements and the destination is a 64-bit register, only the first two elements (index 0 and 1) are relevant to fill the 64-bit destination effectively, or the instruction is used to move a specific 32-bit lane into a 64-bit register with the upper bits zeroed or handled according to the specific opcode variant. In the case of `VEXTRACTI32x8`, it specifically extracts a 64-bit doubleword from a 256-bit vector; the "32x8" notation refers to the source layout, but the extraction result is a single 64-bit value.
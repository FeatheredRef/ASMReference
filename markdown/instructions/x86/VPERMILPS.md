> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VPERMILPS

Permutes single-precision floating-point values from a source to a destination based on an immediate byte. For each of the four destination slots, the instruction selects a 32-bit floating-point value from the source based on the index specified by the corresponding 2-bit field in the immediate.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm reg | xmm reg |
| m128 | xmm reg |

DO NOT support LOCK

This instruction SHALL be executed in 64-bit mode or 32-bit mode. It is NOT available in 16-bit mode. It REQUIRES the AVX instruction set extension.

Ensure that the destination register is not the same as the source register if using a memory operand to avoid potential dependency stalls, although the architecture handles this. The immediate value MUST be provided as a byte; invalid indices beyond the range of the source XMM register are NOT applicable as the immediate is masked to 2 bits per slot.
> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# KSHIFTLB

Shifts the bytes in the source operand to the left by the count specified in the second operand. The shift is performed on each byte individually; bits shifted out of the high end of a byte are discarded, and zeros are shifted into the low end of each byte.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm reg | xmm reg |

DO NOT support LOCK

This instruction is only available when the processor is operating in 64-bit mode or compatibility mode. It requires the AVX-512 VL (Vector Length Extensions) subset.

The shift count must be specified as an immediate value. If the immediate value is not provided or is invalid for the operand size, the instruction will not assemble. The operation is performed element-wise on the bytes within the xmm register; it does not shift bits across byte boundaries.
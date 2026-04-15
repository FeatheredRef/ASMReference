> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VPRORVQ

Rotates the 64-bit quadwords of the destination operand to the right by the count specified in the source operand.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| imm | zmm / ymm / xmm |
| reg | zmm / ymm / xmm |

DO NOT support LOCK

This instruction is available only in 64-bit mode. It requires AVX-512 support; specifically, it is part of the AVX-512 Foundation instructions.

The rotation count is masked to 6 bits (0-63) for 64-bit elements; any bits beyond the 6th bit in the source operand SHALL be ignored. When using a register as the count source, only the low-order 6 bits of the specified register are used. Failure to mask the immediate value or register operand manually is unnecessary as the hardware performs this masking.
> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VPBLENDMB

Selects bytes from two source operands based on a mask provided by an immediate value and stores the result in the destination operand. For each byte in the destination, if the corresponding bit in the immediate is 1, the byte from the first source is selected; if the bit is 0, the byte from the second source is selected.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm/ymm/zmm reg | xmm/ymm/zmm reg |
| xmm/ymm/zmm reg | xmm/ymm/zmm reg |
| imm | #I |

DO NOT support LOCK

This instruction is available only in 64-bit mode. It requires AVX support. The instruction is not supported in compatibility mode for 32-bit operating systems if the AVX feature flag is not enabled in the processor.

The immediate value is used as a mask; since the mask size is limited to 8 bits, this instruction specifically operates on the lowest 8 bytes (64 bits) of the vector registers. If YMM or ZMM registers are used, the upper bits of the destination register are zeroed. To avoid unexpected data loss in the upper portions of a vector register, the user MUST ensure that the target register is treated as a 64-bit value or manually handle the upper bits.
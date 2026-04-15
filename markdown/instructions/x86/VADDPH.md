> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VADDPH

Adds packed signed 16-bit floating-point values from two sources and stores the result in a destination.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm/ymm/zmm register | xmm/ymm/zmm register |
| m16 | xmm/ymm/zmm register |

DO NOT support LOCK

This instruction is available only in 64-bit mode or 32-bit mode. It is NOT supported in compatibility mode.

The instruction requires the AVX-512 F (Foundation) instruction set to be enabled. If the destination register is the same as one of the source registers, the operation is performed in-place.

To avoid unexpected behavior, ensure that the destination register is properly zero-extended if using YMM or ZMM registers to prevent leakage of stale data from the upper bits of the register. Results are subject to the floating-point control word (FCW) settings for rounding and precision.
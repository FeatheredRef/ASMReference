> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VFNMSUB231PD

Computes the negative subtraction of two double-precision floating-point values. It calculates the result as $-(a - b)$, which is mathematically equivalent to $b - a$, where $a$ is the first source operand and $b$ is the second source operand.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm/ymm/zmm reg | xmm/ymm/zmm reg |
| m64/m128/m256/m512 | xmm/ymm/zmm reg |

DO NOT support LOCK

This instruction is only available in 64-bit mode or 32-bit mode. It requires AVX or AVX-512 support depending on the register width used (ymm or zmm).

The instruction adheres to the floating-point control word settings. It SHALL trigger floating-point exceptions based on the result: #O for overflow, #U for underflow, and #P for inexact results. If the operands are signaling NaNs, it SHALL trigger #I.

When using zmm registers, the instruction operates on the elements defined by the mask register if the EVEX encoding is used; failure to properly set the mask may lead to unexpected values in the destination register elements. Memory operands MUST be naturally aligned to the size of the accessed data to avoid performance penalties or faults in certain alignment-strict environments.
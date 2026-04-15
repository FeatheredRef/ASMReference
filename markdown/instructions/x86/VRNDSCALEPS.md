> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VRNDSCALEPS

Rounds the packed single-precision floating-point values of a source to the nearest scalable integer, scaling the result by a power of two specified by an immediate value.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| xmm/ymm/zmm reg | xmm/ymm/zmm reg |
| m128/m256/m512 | xmm/ymm/zmm reg |

DO NOT support LOCK

This instruction is available only in 64-bit mode. It requires the AVX-512 foundation extensions to be enabled.

The scale factor is encoded as an immediate value (`imm8`). If the scale factor is not within the valid range for the specified rounding mode, the behavior is implementation-defined or may result in an exception. Users SHALL ensure the immediate value corresponds to a valid power-of-two scale to avoid unexpected results.

Rounding is controlled by the rounding mode specified in the EVEX prefix. If the rounding mode is set to "Round to Nearest", the instruction follows the standard IEEE 754 rounding rules for the scaled result.

Possible exceptions:
- #D: Occurs if any of the floating-point operands are denormalized.
- #O: Occurs if the result of the scaling operation exceeds the maximum representable value for the destination format.
- #P: Occurs if the result is inexact.
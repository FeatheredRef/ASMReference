> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VMINSH

Computes the minimum of the lower 64 bits of two packed floating-point values (single precision) and stores the result in the destination. This instruction operates on the lower 128 bits of the YMM registers, treating them as two packed single-precision floating-point values, but only the minimum of the lower elements is relevant for the "SH" (Single Half) variant in specific AVX-512 contexts, typically updating the lower 64 bits of the destination.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| m128 | reg |

DO NOT support LOCK

This instruction SHALL be executed in 64-bit mode. It requires the AVX-512 foundation extensions to be enabled. Use of this instruction in compatibility mode is NOT supported.

Ensure that the destination register is not the same as a source register if masking is not applied, although generally, VMINSH supports destructive sources. Note that if the input contains a NaN, the result is determined by the specific AVX-512 floating-point minimum rules: if one operand is a NaN, the other operand is returned. If both are NaNs, the result is the first operand. This prevents the propagation of Quiet NaNs in specific logic paths.
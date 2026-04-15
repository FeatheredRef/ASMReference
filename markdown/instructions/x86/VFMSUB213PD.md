> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VFMSUB213PD

Computes a fused multiply-subtract operation on double-precision floating-point values. It calculates the result of $(a \times b) - c$ for four pairs of double-precision floating-point numbers, where $a$ is the first source operand, $b$ is the second source operand, and $c$ is the third source operand. The operation is performed with a single rounding step at the end.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| zmm / m128 | zmm |
| zmm / m128 | zmm |
| zmm / m128 | zmm |

DO NOT support LOCK

This instruction is only available when the processor supports the AVX-512 foundation instructions. It requires the processor to be in 64-bit mode or a compatible operating mode that supports AVX-512.

The instruction uses the masking register (k-register) for conditional writing. If masking is applied, the destination elements that are masked out will either be zeroed (if `{z}` is specified) or remain unchanged. Failure to properly initialize the mask register may result in undesired elements being skipped.

The fused multiply-subtract operation avoids intermediate rounding, which prevents precision loss that would occur if a separate `VMULPD` and `VSUBPD` were used. If the result is too large to be represented, it triggers #O; if it is too small, it triggers #U. Resulting precision loss triggers #P.
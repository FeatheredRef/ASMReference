> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VFNMADD213PS

Computes the fused multiply-add operation for single-precision floating-point values. It calculates the result of (-(a * b) + c) for each corresponding element in the source operands, where 'a' is the first source, 'b' is the second source, and 'c' is the third source.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| reg, reg, reg | reg |
| reg, reg, m128 | reg |
| reg, m128, reg | reg |
| m128, reg, reg | reg |

DO NOT support LOCK

This instruction SHALL only be executed on processors that support the AVX and FMA3 instruction sets. It is available in both 64-bit mode and compatibility mode.

The operation is performed as a single fused step; intermediate rounding does not occur between the multiplication and the addition. This prevents precision loss that would occur if `VMULPS` and `VADDPS` were used sequentially.

Failure to ensure that the destination register is not one of the source registers in certain non-destructive forms may lead to unexpected data loss, although the VEX-encoded version typically supports three-operand syntax to avoid this. The instruction MUST be used with properly aligned memory operands to avoid performance penalties or general protection faults depending on the alignment check settings.

#P, #O, #U, #D, #Z are triggered based on the floating-point exception flags defined in the MXCSR register.
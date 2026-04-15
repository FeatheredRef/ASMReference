> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VFNMADD231SH

Performs a fused multiply-add operation on floating-point values. It computes the result as: $dest = -(a \times b) + c$, where $a$ is the first operand, $b$ is the second operand, and $c$ is the third operand. This instruction is specifically designed for single-precision floating-point numbers and supports both scalar and packed operations.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| f32/f128 (reg) | f32/f128 (reg) |
| f32/f128 (reg) | f32/f128 (reg) |
| f32/f128 (mem) | f32/f128 (reg) |

DO NOT support LOCK

This instruction SHALL be executed in 64-bit mode or 32-bit mode. It is NOT supported in compatibility mode. The instruction requires AVX support; if executed on a processor that does not support the AVX ISA extension, an invalid opcode exception SHALL be generated.

To avoid precision loss or incorrect results, ensure the MXCSR register is correctly configured for rounding modes. The operation is performed with a single rounding step at the end, which prevents the intermediate overflow or underflow that would occur if the multiplication and addition were performed as separate instructions. This may lead to results that differ slightly from non-fused sequences of `VMULPS` and `VADDPS`. Floating-point exceptions such as #P, #O, #U, and #D may be signaled based on the final result and the state of the MXCSR register.
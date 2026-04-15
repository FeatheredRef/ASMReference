> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VFNMSUB213SS

Computes the fused multiply-subtract of three scalar single-precision floating-point values. The operation is performed as: $dest = (a \times b) - c$, where the operands are mapped based on the instruction encoding (213 indicates the destination is the third operand).

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| f32, f32, f32 | f32 |
| reg, reg, reg | reg |
| reg, reg, m4 | reg |
| reg, m4, reg | reg |
| m4, reg, reg | reg |

DO NOT support LOCK

This instruction is available only in 64-bit mode or 32-bit mode. It SHALL NOT be used in 16-bit compatibility mode. It requires the AVX CPU feature flag to be supported by the hardware.

To avoid precision loss or unexpected results, be aware that this instruction performs a single rounding step at the end of the operation. This prevents the accumulation of intermediate rounding errors that would occur if using separate `VMULSS` and `VSUBSS` instructions. If the result exceeds the maximum representable value of a f32, it SHALL trigger #O. If the result is too small to be represented, it SHALL trigger #U. Results that cannot be represented exactly SHALL trigger #P.
> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VFMADD132PH

Performs a fused multiply-add operation on packed half-precision floating-point values. The operation computes the product of the second and third operands and adds the result to the first operand: $dest = (src2 \times src3) + src1$.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| reg, reg, reg | reg |
| reg, reg, m128 | reg |
| m128, reg, reg | reg |

DO NOT support LOCK

This instruction is available only in 64-bit mode or 32-bit mode. It requires the AVX-512 FP16 extension to be supported by the processor. The instruction operates on ZMM, YMM, or XMM registers.

To avoid precision loss or unexpected #I, ensure that the input operands are valid half-precision floating-point numbers. The operation is performed with infinite precision internally; rounding occurs only once at the final addition step. If the result exceeds the representable range of f16, it SHALL trigger #O. If the result is too small to be represented, it SHALL trigger #U. If the result is not exactly representable, it SHALL trigger #P.
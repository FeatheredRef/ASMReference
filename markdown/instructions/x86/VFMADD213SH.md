> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VFMADD213SH

Computes the fused multiply-add operation using packed single-precision floating-point values. The instruction calculates the product of two operands and adds a third operand to the result, following the formula: $dest = (src1 \times src2) + src3$. The "213" notation specifies that the destination is the first operand, the second operand is the second source, and the third operand is the third source.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| reg | reg |
| m32 | reg |

DO NOT support LOCK

This instruction is available only in 64-bit mode or compatibility mode. It requires the AVX support bit to be enabled in the CPU.

The operation is performed as a single fused step, meaning only one rounding error occurs at the final step, which prevents intermediate precision loss. If the precision control in the MXCSR register is set to truncate, the result is rounded toward zero.

The instruction may trigger the following exceptions:
- #I: If an operand is a Signaling NaN.
- #O: If the result exceeds the maximum representable f32 value.
- #U: If the result is smaller than the minimum representable f32 value.
- #P: If the result is not exact.

To avoid performance degradation due to alignment issues, memory operands (m32) SHOULD be aligned to the boundary of the vector size (e.g., 128-bit or 256-bit) to prevent penalties or faults depending on the memory model.
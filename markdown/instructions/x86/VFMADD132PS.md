> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VFMADD132PS

Performs a fused multiply-add operation on single-precision floating-point values. It computes the product of the second and third operands and adds the result to the first operand. The operation is performed as a single step with only one rounding error at the end.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg, reg, reg | reg |
| m32, reg, reg | reg |
| reg, m32, reg | reg |
| reg, reg, m32 | reg |

DO NOT support LOCK

This instruction SHALL be executed in 64-bit mode or compatibility mode. It REQUIRES the FMA3 feature set to be enabled in the processor.

To avoid precision loss or incorrect results, ensure that the MXCSR register is configured with the desired rounding mode, as the fused operation only rounds once after the summation. Failure to account for the 132 operand order (dest = dest + (src2 * src3)) may lead to logic errors in mathematical expressions compared to standard VFMADD213PS or VFMADD231PS instructions. Results may trigger #P, #O, #U, or #D exceptions based on the floating-point environment.
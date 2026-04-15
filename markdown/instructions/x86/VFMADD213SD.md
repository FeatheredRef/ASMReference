> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VFMADD213SD

Performs a fused multiply-add operation on scalar double-precision floating-point values. It computes the product of the first and second operands, adds the result to the third operand, and stores the result in the destination operand. The operation is performed with a single rounding step at the end.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| f64, f64, f64 | f64 |
| reg, reg, reg | reg |
| reg, reg, m8 | reg |
| reg, m8, reg | reg |
| m8, reg, reg | reg |

DO NOT support LOCK

This instruction SHALL only be executed in 64-bit mode or compatibility mode. It REQUIRES the AVX support bit to be enabled in the processor.

The instruction uses the VEX prefix, which prevents the automatic merging of the upper 64 bits of the YMM register; therefore, the upper bits of the destination xmm register are zeroed. To avoid precision loss or unexpected results, the user MUST ensure that the floating-point control word is correctly configured, as the operation is subject to the rounding mode and exception flags defined in the MXCSR register. Potential exceptions include #I, #D, #O, #U, and #P.
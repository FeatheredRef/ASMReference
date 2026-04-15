> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VFNMADD213PD

Computes the sum of the product of the first two operands and the third operand, then multiplies the result by -1. The operation is performed as: $dest = -(a \times b + c)$.

The following table covers the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| reg, m64 | reg |

DO NOT support LOCK

This instruction is only available in 64-bit mode or 32-bit mode. It requires the AVX support feature.

The destination register is overwritten by the result; therefore, the destination must not be used as a source if the original value is required. The operation is subject to floating-point exception flags such as #P, #O, #U, #D, and #Z according to the IEEE 754 standard and the MXCSR register settings. Precision is handled based on the current rounding mode specified in the MXCSR register.
> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VFNMSUB231PH

Subtracts the product of two half-precision floating-point values from a third half-precision floating-point value, negates the result, and stores it in the destination. The operation performs: `dest = -(src1 - (src2 * src3))`.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| f16, f16, f16 | f16 |
| reg, reg, reg | reg |
| m16, reg, reg | reg |

DO NOT support LOCK

This instruction requires the AVX-512 FP16 extension. It shall only operate on zmm or ymm registers in 64-bit mode or compatibility mode. Execution in 32-bit mode is not supported.

To avoid precision loss or unexpected results, ensure that the destination register is not used as a source operand unless a non-destructive operation is intended. The instruction is subject to the rounding mode specified in the MXCSR register. If the operation results in a value that cannot be represented in f16 format, it may trigger #O, #U, or #P exceptions depending on the floating-point control word.
> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# HADDPS

Adds corresponding elements of two packed single-precision floating-point vectors, then performs a horizontal add of the adjacent pairs of elements within each resulting vector.

The following table covers the supported source and destination operands:

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| m128 | reg |

DO NOT support LOCK

This instruction is available in 64-bit mode and compatibility mode. It requires the SSE3 instruction set extension to be supported by the processor.

The destination register is overwritten by the result; therefore, the destination register cannot be used as a source operand if the original values must be preserved.

The operation is subject to the floating-point control word settings in the MXCSR register. Specifically, the rounding mode determines how results are rounded. If an operation results in an overflow, it SHALL trigger #O; if it results in a loss of precision, it SHALL trigger #P. Denormalized operands SHALL trigger #D.
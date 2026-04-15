> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VFMSUB213PH

Performs a fused multiply-subtract operation on packed half-precision floating-point values. It computes the result of (a * b) - c for each corresponding element of the source operands.

The following table specifies the supported source and destination types.

| source | destination(s) |
| :--- | :--- |
| reg | reg |

DO NOT support LOCK

This instruction is available only in 64-bit mode or compatibility mode. It requires support for the AVX-512 FP16 extensions.

The operation is performed according to the IEEE 754-2008 standard for half-precision floating-point arithmetic. The intermediate product (a * b) is calculated with infinite precision before the subtraction of c occurs, avoiding an intermediate rounding step.

The result is subject to the rounding mode specified in the MXCSR register. The following floating-point exceptions may be triggered: #D, #O, #U, and #P. Memory operands are not supported for this instruction; operands MUST be provided in xmm or zmm registers.
> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VFNMSUB132PH

Subtracts the product of two floating-point half-precision (f16) values from a third floating-point half-precision value, then negates the result. The operation is performed as: $dest = -(src1 - (src2 \times src3))$.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg | reg |

DO NOT support LOCK

This instruction requires AVX-512 FP16 support. It is only available when the processor is operating in 64-bit mode.

The operation follows the IEEE 754 standard for half-precision floating-point arithmetic. Precision and rounding are controlled by the MXCSR register. If the result is too large to be represented in f16 format, it results in #O. If the result is too small, it results in #U. Any inexact result triggers #P. Denormalized operands may trigger #D depending on the flush-to-zero (FTZ) and denormals-are-zero (DAZ) flags in MXCSR.
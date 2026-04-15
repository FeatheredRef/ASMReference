> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VFMSUB213SS

Subtracts the product of two floating-point values from a third floating-point value, using scalar single-precision floating-point values. The operation is defined as: $dest = src1 - (src2 \times src3)$.

The following table covers the supported sources and destinations:

| source | destination(s) |
| :--- | :--- |
| xmm reg | xmm reg |
| m4 | xmm reg |

DO NOT support LOCK

This instruction is only available when the AVX512F feature is enabled. It operates exclusively in 64-bit mode or compatibility mode.

To avoid precision loss or unexpected results, ensure that the destination register is not used as one of the source operands unless the fused multiply-subtract behavior is intended, as the operation is performed with a single rounding step at the end. The instruction is subject to floating-point exceptions including #D, #O, #U, and #P based on the MXCSR register settings.
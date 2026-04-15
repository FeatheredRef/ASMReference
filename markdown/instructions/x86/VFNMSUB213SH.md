> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VFNMSUB213SH

Subtracts the product of two floating-point source operands from a third source operand, then multiplies the result by a scalar and stores the result in a destination operand. Specifically, it computes $dest = (src1 - (src2 \times src3)) \times \text{scalar}$.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| fN (reg) | fN (reg) |

DO NOT support LOCK

This instruction is only available when the processor is operating in 64-bit mode or compatibility mode. It requires the AVX-512 foundation and the AVX-512 Floating-Point (AVX512F) extension.

The operation is subject to the rounding mode and exception handling specified in the MXCSR register. If the result exceeds the representable range of the destination format, an #O exception may be triggered. Inexact results will trigger the #P exception. Memory operands are not supported; operands SHALL be stored in zmm or ymm registers.
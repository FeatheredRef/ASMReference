> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VFMSUB132SS

Performs a fused multiply-subtract operation on the lowest 32-bit floating-point scalar elements of the first source operand and the second source operand, then subtracts the result from the third source operand. The result is stored in the destination operand.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |
| xmm | xmm |
| m4 | xmm |

DO NOT support LOCK

This instruction SHALL be used only in 64-bit mode or 32-bit mode. It is NOT supported in compatibility mode if the processor does not support the FMA3 instruction set.

To avoid precision loss or incorrect results, the user MUST ensure that the floating-point control word is correctly configured, as the operation is performed with infinite precision before rounding to the destination format. Failure to account for the fused nature of the operation may lead to different results compared to separate MULSS and SUBSS instructions.
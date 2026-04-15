> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VFNMSUB231PS

Subtracts the product of the second and third source operands from the first source operand for each corresponding 32-bit floating-point element. The operation is defined as: $destination = source1 - (source2 \times source3)$.

The following table describes the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| xmm/ymm/zmm | xmm/ymm/zmm |
| m32/m64/m128/m256/m512 | xmm/ymm/zmm |

DO NOT support LOCK

This instruction is only available in 64-bit mode or 32-bit mode when the AVX-512 foundation is supported. It requires the `VEX` or `EVEX` prefix. If the `EVEX` prefix is used, the instruction may employ masking (k-registers) to conditionally update elements of the destination register.

The instruction supports the execution of the operation using the specified rounding mode in the MXCSR register. It may trigger floating-point exceptions including #D, #O, #U, and #P based on the result of the fused multiply-subtract operation. To avoid precision loss, note that the operation is performed with infinite precision before rounding to the destination format. Use of the `EVEX` encoded version is REQUIRED for operations on `zmm` registers.
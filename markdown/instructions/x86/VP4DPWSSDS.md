> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VP4DPWSSDS

This instruction performs a packed double-precision floating-point subtract with a double-precision saturation, processing elements in a vectorized manner. It subtracts the second operand from the first, and the result is saturated to the maximum or minimum representable double-precision floating-point value if the result overflows.

The following table describes the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| zmm / ymm / xmm | zmm / ymm / xmm |
| m32 / m64 / m128 / m256 / m512 | zmm / ymm / xmm |

DO NOT support LOCK

This instruction is ONLY available in 64-bit mode or compatibility mode. It REQUIRES support for AVX-512 and the specific foundation or extension subset that implements the `VP4DPWSSDS` opcode.

The instruction's behavior is governed by the MXCSR register settings. Specifically, the rounding mode defined in MXCSR SHALL be applied to the subtraction operation. If the result exceeds the representable range of a double-precision float, it SHALL be saturated rather than resulting in $\pm\infty$, provided the specific saturation flags are active. Failure to ensure the correct AVX-512 feature set is enabled in the CPU will result in an invalid opcode exception.
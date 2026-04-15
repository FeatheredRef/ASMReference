> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VFMADD132PD

Sets the destination operand to the sum of the product of the second and third operands and the first operand. The operation is performed as: $dest = (src2 \times src3) + src1$.

The following table covers the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| reg, m128, m256, m512 | reg |

DO NOT support LOCK

This instruction requires AVX, AVX2, or AVX-512 support depending on the operand size used. It is not available in compatibility mode if the AVX-512 extensions are utilized. The instruction shall be executed in 64-bit mode or 32-bit mode.

The destination register is used as an implicit source (the addend). If the destination register is not the same as the first source operand, the result will overwrite the destination. To avoid unintended data loss, the destination register MUST be backed up if its original value is required after the operation.

Floating-point exceptions may occur:
- #I: occurs if any operand is a NaN or if the operation involves $\infty \times 0$.
- #Z: not applicable.
- #D: occurs if an operand is denormalized or the result is denormalized.
- #O: occurs if the result exceeds the maximum representable value.
- #U: occurs if the result is smaller than the minimum representable value.
- #P: occurs if the result is rounded.
> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VFNMSUB213PS

Computes the fused multiply-subtract of three single-precision floating-point operands. The operation performs the calculation $dest = (a \times b) - c$, where the operands are selected based on the 213 pattern: the first source operand is multiplied by the second source operand, and the result is subtracted from the third source operand.

The following table covers the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| m32 | reg |

DO NOT support LOCK

This instruction SHALL be executed in 64-bit mode or compatibility mode. It requires support for the AVX extension.

The operation is performed with a single rounding step at the end, which prevents intermediate rounding errors. Users MUST ensure that the destination register is not used as a source if they intend to preserve the original value, as the result OVERWRITES the destination. If the destination is used as the third operand ($c$), the operation behaves as $dest = (a \times b) - dest$.

Floating-point exceptions may occur based on the result:
- #I: Invalid operation (e.g., $\infty \times 0$).
- #D: Denormalized operand.
- #O: Numeric overflow.
- #U: Numeric underflow.
- #P: Inexact result.
> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VFNMSUB132PD

Subtracts a floating-point value from the product of two floating-point values, then negates the result. Specifically, it computes the result as: $-(a \times b) + c$.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| f64, f64, f64 | f64 |
| m64, f64, f64 | f64 |

DO NOT support LOCK

This instruction is available only in 64-bit mode or compatibility mode. It requires a processor supporting the AVX512 foundation.

The instruction uses the destination register as the addend ($c$), and the two source operands as the multipliers ($a$ and $b$). If the destination register is also used as one of the source operands, the original value is overwritten by the result.

The operation is subject to floating-point exceptions: #O, #U, #P, and #D. If the input values are signaling NaNs, #I shall be raised. The rounding mode is governed by the MXCSR register.
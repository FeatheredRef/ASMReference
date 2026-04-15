> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VFNMSUB213SD

This instruction performs a fused multiply-subtract operation on scalar double-precision floating-point values. It computes the result using the formula: $result = (a \times b) - c$, where the operands are mapped as $a = \text{op1}$, $b = \text{op2}$, and $c = \text{op3}$. The operation is performed with a single rounding step at the end to maintain precision.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| f64, f64, f64 | f64 |
| reg, reg, reg | reg |
| reg, reg, m8 | reg |
| reg, m8, reg | reg |
| m8, reg, reg | reg |

DO NOT support LOCK

This instruction is available only in 64-bit mode or 32-bit mode when the AVX2 instruction set is enabled. It requires the processor to support the FMA3 extension. It SHALL NOT be used in compatibility mode if the hardware does not explicitly support AVX2.

The destination register is overwritten by the result, meaning one of the source operands is typically destroyed unless a three-operand non-destructive form is used. Ensure that the `MXCSR` register is correctly configured to handle floating-point exceptions; otherwise, the operation may result in #P, #O, #U, or #Z depending on the input values. If any operand is a Signaling NaN, a #I exception shall be raised.
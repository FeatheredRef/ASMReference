> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# HSUBPS

Subtracts the minimum of two packed single-precision floating-point values from a third packed single-precision floating-point value. For each corresponding element in the source operands, the instruction calculates `result = operand1 - min(operand2, operand3)`.

The following table describes the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| xmm, m128 | xmm |

DO NOT support LOCK

This instruction is only available in 64-bit mode or compatibility mode. It requires SSE3 support.

The destination register is also used as the first source operand. If the precision control field in the MXCSR register is set to denormals-are-zero (DAZ) or flush-to-zero (FTZ), the behavior of the subtraction and the minimum operation may result in #D, #O, #U, or #P depending on the floating-point environment. Failure to initialize the MXCSR register consistently across threads may lead to divergent results when processing denormalized values.
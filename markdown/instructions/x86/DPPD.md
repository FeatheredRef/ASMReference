> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# DPPD

Determines the sign of the difference between two double-precision floating-point values. It compares the source operands and sets the destination operand to 1.0 if the first operand is greater than the second, -1.0 if the first operand is less than the second, and 0.0 if the operands are equal, unordered, or if either operand is a NaN.

The table below covers the supported source and destinations.

| source | destination(s) |
| :--- | :--- |
| f64, f64 | f64 |
| m8, m8 | f64 |

DO NOT support LOCK

This instruction is only available when the processor is operating in 64-bit mode or compatibility mode. It requires the presence of the SSE4.1 instruction set extension.

The destination operand MUST be an XMM register; memory-to-memory operations are NOT supported. If either source operand is a NaN, the result is 0.0. The comparison is performed according to the IEEE 754 standard for double-precision floating-point numbers.
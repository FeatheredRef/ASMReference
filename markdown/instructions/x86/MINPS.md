> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# MINPS

Subtracts the minimum of two packed single-precision floating-point values from each corresponding element of two XMM operands.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| r128 | r128 |
| m16 | r128 |

DO NOT support LOCK

This instruction is available in 64-bit mode and compatibility mode. It requires SSE support.

The destination register must be a 128-bit XMM register. Memory operands MUST be 16-byte aligned to avoid general protection faults or alignment check exceptions.

The operation follows the IEEE 754 standard for floating-point arithmetic. It may trigger floating-point exceptions including #P, #O, #U, or #D depending on the input values and the MXCSR register settings.
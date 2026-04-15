> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# FABS

Computes the absolute value of the source operand by clearing the sign bit of the floating-point value.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| m32 | #I |
| m64 | #I |
| f32 | f32 |
| f64 | f64 |

DO NOT support LOCK

The instruction is available in 64-bit mode and compatibility mode. It operates exclusively on XMM registers or memory locations that are interpreted as floating-point values. The instruction SHALL NOT be used with general-purpose registers.

To avoid incorrect results, ensure that the destination register is of the same size as the source operand (f32 or f64). Using an incompatible size for the destination will result in an invalid operation. Precision (#P) may be triggered if the operation results in an inexact result according to the floating-point control word.
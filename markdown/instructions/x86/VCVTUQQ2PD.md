> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VCVTUQQ2PD

Converts an unsigned qword integer value to a double-precision floating-point value.

The following table describes the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| m64 | f64 |
| r64 | f64 |

DO NOT support LOCK

This instruction is available only in 64-bit mode. It is NOT supported in compatibility mode.

The destination operand MUST be an XMM register. If the source is a memory operand, it SHALL be aligned to the appropriate boundary according to the memory model; otherwise, an alignment check exception may occur.

The instruction may set the #P (Inexact result) exception flag if the rounded result cannot be represented exactly in the double-precision format. Precision is handled according to the rounding control in the MXCSR register.
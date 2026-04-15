> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# CVTPI2PD

Converts a signed integer value to a double-precision floating-point value.

The following table covers the supported source and destinations:

| source | destination(s) |
| :--- | :--- |
| m32 | f64 |
| r32 | f64 |

DO NOT support LOCK

This instruction is available in 64-bit mode and compatibility mode. It requires the SSE2 instruction set.

The source operand must be a signed 32-bit integer (i32). The operation performs a conversion from a signed integer to a double-precision floating-point format. If the result cannot be represented exactly in the destination format, the result is rounded according to the rounding control word in the MXCSR register. A precision exception (#P) may be signaled if the result is rounded.
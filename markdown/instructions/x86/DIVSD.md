> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# DIVSD

Divides the scalar double-precision floating-point value in the destination operand by the scalar double-precision floating-point value in the source operand.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm8 | xmm8 |
| m8 | xmm8 |

DO NOT support LOCK

The instruction is available in 64-bit mode and compatibility mode. It requires the SSE unit to be present.

The destination register MUST be an XMM register. If the result is too large to be represented in the destination format, #O is signaled. If the divisor is zero, #Z is signaled. If the result is too small to be represented, #U is signaled. Precision exceptions #P may occur if the result requires rounding. All operations are subject to the rounding mode and exception mask settings in the MXCSR register.
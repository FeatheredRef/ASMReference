> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# SQRTSS

Computes the square root of a scalar single-precision floating-point value.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |
| m32 | xmm |

DO NOT support LOCK

This instruction is available in 64-bit mode and compatibility mode. It requires SSE support.

The instruction shall follow the precision and rounding control set in the MXCSR register. If the source operand is a signaling NaN, #I is signaled. If the source operand is negative, #I is signaled and a QNaN is returned. If the source operand is zero, the result is zero and no exception is signaled. Results that cannot be represented in the destination format shall signal #P.
> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VSQRTSH

Computes the square root of a packed signed short floating-point value.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| m16 | r128 |
| r128 | r128 |

DO NOT support LOCK

This instruction is available only when using the AVX extension. It operates on packed 16-bit floating-point values (half-precision).

If the source operand contains a negative value, the result is a NaN (Not-a-Number) and the #I exception is signaled. If the source is zero, the result is zero and no exception is signaled. If the source is a NaN, the result is a NaN.

Precision control: The instruction is subject to the rounding control bits in the MXCSR register. If the result is not exact, the #P exception is signaled.
> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# SQRTSD

Computes the square root of a scalar double-precision floating-point value.

The following table describes the supported sources and destinations.

| source | destination(s) |
| :--- | :--- |
| m8 | xmm |
| xmm | xmm |

DO NOT support LOCK

This instruction is available in both 64-bit mode and compatibility mode. It requires the SSE3 extension to be supported by the processor.

If the source operand is a signaling NaN, #I is generated. If the source operand is negative, #I is generated and a qNaN is returned. If the source operand is negative zero, the result is negative zero and no exception is generated.

The instruction behavior is governed by the rounding mode specified in the MXCSR register. If the result cannot be represented exactly in the destination f64 format, #P is generated.
> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# SQRTPD

Computes the square root of a double-precision floating-point value.

The following table specifies the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| m64/xmm | xmm |

DO NOT support LOCK

This instruction is available in 64-bit mode and compatibility mode. It requires SSE3 support.

If the source operand is a signaling NaN, #I is raised. If the source operand is negative, #I is raised and the result is a qNaN. If the source operand is negative zero, the result is negative zero and no exception is raised.

To avoid precision loss or unexpected exceptions, ensure the input is a non-negative f64. The instruction will trigger #P if the result is not exactly representable.
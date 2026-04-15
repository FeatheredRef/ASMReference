> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# CVTPD2PI

Converts the two packed double-precision floating-point values to two packed signed integers. The conversion is performed by rounding the values according to the rounding control in the MXCSR register.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| m128 | xmm |
| xmm | xmm |

DO NOT support LOCK

This instruction is only available in 64-bit mode. It requires SSE2 support.

If the converted value is too large to be represented as a signed 32-bit integer, the result is the indeterminate value (integer INDEF), and a #O exception is generated. If the input is a NaN, the result is the indeterminate value (integer INDEF). Ensure that the MXCSR rounding mode is correctly configured before execution to avoid unexpected precision loss or incorrect rounding behavior.
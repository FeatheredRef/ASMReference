> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VRNDSCALEPD

This instruction rounds a 64-bit floating-point value to the nearest integer according to the specified rounding mode and scales the result by a power of two. It operates on packed double-precision floating-point values.

The following table describes the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| zmm/ymm/xmm | zmm/ymm/xmm |
| m64 | zmm/ymm/xmm |

DO NOT support LOCK

This instruction is available only in 64-bit mode or 32-bit mode. It is not supported in compatibility mode.

The rounding mode is determined by the immediate operand, which overrides the rounding control field in the MXCSR register. If the immediate specifies a rounding mode that is not supported by the hardware, the result is undefined.

The scaling factor is encoded as a signed integer within the immediate. If the scaling factor results in a value that exceeds the maximum representable value of a double-precision float, #O is raised. If the result is too small to be represented, #U is raised. Precision loss during rounding triggers #P.
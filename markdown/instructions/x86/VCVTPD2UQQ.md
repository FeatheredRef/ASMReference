> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VCVTPD2UQQ

Converts packed double-precision floating-point values to packed unsigned 64-bit integers with rounding. The instruction converts each double-precision floating-point value in the source to an unsigned 64-bit integer according to the rounding mode specified in the MXCSR register, and stores the results in the destination.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| xmm/ymm | xmm/ymm |
| m128/m256 | xmm/ymm |

DO NOT support LOCK

This instruction is only available when the processor is operating in 64-bit mode. It is NOT supported in compatibility mode.

The result of the conversion is subject to the following behaviors:
- If the source value is NaN, the result is set to the indefinite value (all bits 1).
- If the source value is $\pm\infty$ or exceeds the maximum representable value of a u64, the result is set to the maximum representable u64 value.
- If the source value is less than 0, the result is set to 0.

The instruction may trigger the following exceptions:
- #P: If the result is inexact.
- #O: If the converted value is too large to be represented in the destination format.
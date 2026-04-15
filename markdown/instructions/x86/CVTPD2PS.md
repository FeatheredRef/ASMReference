> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# CVTPD2PS

Converts a packed double-precision floating-point value to a packed single-precision floating-point value with rounding.

The following table describes the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| m128 | xmm |
| xmm | xmm |

DO NOT support LOCK

This instruction is available in 64-bit mode and 32-bit mode. It requires SSE3 support.

The instruction converts two double-precision floating-point values (f64) to two single-precision floating-point values (f32). The result is stored in the lower 64 bits of the destination register; the upper 64 bits of the destination register are cleared.

Rounding is performed according to the rounding-control mode in the MXCSR register. If the source value is too large to be represented as a single-precision value, the result is set to infinity and #O is signaled. If the result is not exactly representable, #P is signaled.
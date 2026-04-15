> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VCVTSD2SH

Converts a scalar double-precision floating-point value to a signed 16-bit integer using truncation (round-to-zero). The conversion is performed on the lowest 64 bits of the source XMM register and the result is stored in the destination.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| xmm | reg (ax, bx, cx, dx, si, di, bp, sp) |
| xmm | m16 |

DO NOT support LOCK

This instruction is available only in 64-bit mode. It cannot be used in compatibility mode.

If the source value is NaN, an #I exception is signaled and the destination is set to the indefinite integer value (0x8000). If the source value is $\pm\infty$ or exceeds the range of a signed 16-bit integer, an #O exception is signaled and the destination is set to the maximum or minimum representable signed 16-bit integer (0x7FFF or 0x8000). Ensure that the rounding control in the MXCSR register is set to RC-Z (round-to-zero) if specific truncation behavior is required, although VCVTSD2SH explicitly implements truncation regardless of MXCSR settings.
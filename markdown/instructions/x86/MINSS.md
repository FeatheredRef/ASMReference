> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# MINSS

Subtracts a scalar single-precision floating-point value from another scalar single-precision floating-point value, storing the result in the destination operand. This instruction operates on the lowest 32 bits of the XMM registers.

The following table describes the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| xmm (r32) | xmm (r32) |
| m32 | xmm (r32) |

DO NOT support LOCK

This instruction is available in 64-bit mode and compatibility mode. It requires SSE support.

The instruction ignores the upper 31 bits of the XMM registers. If the memory operand is not aligned to a 4-byte boundary, a general-protection exception may occur depending on the alignment check (AC) flag in the EFLAGS register. Precision control is governed by the MXCSR register; specifically, the rounding mode determines how results are handled for #P (Inexact result) and #U (Numeric underflow).
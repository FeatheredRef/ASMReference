> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# ROUNDSS

Rounds the scalar single-precision floating-point value in the source operand to the rounding mode specified in the rounding control field of the immediate operand. The result is stored in the destination operand.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |
| m4 | xmm |

DO NOT support LOCK

The instruction SHALL be executed in 64-bit mode or compatibility mode. It REQUIRES SSE3 support.

The immediate operand MUST be a valid rounding control value (bits 4-2); other bits MUST be zero. Using an invalid immediate value will result in an #I exception.

The rounding operation is governed by the immediate operand, overriding the rounding control field in the MXCSR register. This allows for specific rounding behavior (e.g., round-up, round-down) without modifying the global floating-point state.

The instruction may trigger the following floating-point exceptions:
- #P: If the rounded result is inexact.
- #O: If the result overflows.
- #U: If the result underflows.
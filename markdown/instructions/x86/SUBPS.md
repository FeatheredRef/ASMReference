> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# SUBPS

Subtracts four single-precision floating-point numbers from another set of four single-precision floating-point numbers and stores the results.

The following table covers what the source and destinations can be:

| Source | Destination(s) |
| :--- | :--- |
| xmm reg | xmm reg |
| m128 | xmm reg |

DO NOT support LOCK

This instruction requires SSE support. It is available in both 64-bit mode and compatibility mode.

The operation is performed based on the rounding mode specified in the MXCSR register. If any input operand is a signaling NaN, a #O exception MAY be generated depending on the floating-point environment.

To avoid precision loss or unexpected behavior, ensure that the memory operands are aligned on 16-byte boundaries; otherwise, a general protection fault MAY occur or performance SHALL be degraded depending on the processor implementation. All elements must be valid single-precision floating-point numbers to avoid #I.
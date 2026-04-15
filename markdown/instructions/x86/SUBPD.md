> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# SUBPD

Subtracts the lower 64 bits of the first operand from the lower 64 bits of the second operand and the upper 64 bits of the first operand from the upper 64 bits of the second operand. The result is stored in the destination operand.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| xmm reg | xmm reg |
| m128 | xmm reg |

DO NOT support LOCK

This instruction is available in 64-bit mode and 32-bit mode. It requires SSE3 support.

The instruction is subject to the floating-point control word settings in the MXCSR register. If the precision control is set to truncate, the result MAY be rounded according to the current rounding mode. Failure to handle floating-point exceptions MAY lead to #P, #O, #U, or #D depending on the resulting value. Precision issues MAY occur if the result cannot be represented exactly in f64 format.
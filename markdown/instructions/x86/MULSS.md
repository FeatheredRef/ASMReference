> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# MULSS

Multiplies the low 32 bits of the first source operand by the low 32 bits of the second source operand and stores the result in the destination operand.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| xmm reg | xmm reg |
| m32 | xmm reg |
| xmm reg | xmm reg |

DO NOT support LOCK

This instruction is available in 64-bit mode and 32-bit mode. It requires SSE3 support.

The operation is performed on the lowest 32 bits of the XMM registers; the upper 96 bits of the destination register remain unchanged. If the MXCSR register is configured for "deny" denormal operations, the instruction MAY trigger #D. Precision exceptions #P, overflow #O, and underflow #U MAY occur depending on the result and the MXCSR rounding and exception masks.
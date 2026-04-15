> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# ADDPS

Adds four packed single-precision floating-point values from a source to four packed single-precision floating-point values in a destination.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |
| m128 | xmm |

DO NOT support LOCK

This instruction is available in 64-bit mode, 32-bit mode, and compatibility mode. It requires the SSE extension to be supported by the processor.

Alignment of memory operands SHALL be 16-byte aligned unless the processor supports unaligned memory access; otherwise, a general-protection exception is generated.

Floating-point exceptions may be triggered:
- #D: If any input operand is a denormalized value.
- #O: If any result exceeds the maximum representable single-precision value.
- #U: If any result is smaller than the minimum representable single-precision value.
- #P: If the result cannot be represented exactly.
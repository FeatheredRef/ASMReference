> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# MOVUPD

Moves unaligned packed single-precision floating-point values from a source to a destination.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| xmm reg | xmm reg |
| xmm reg | m128 |
| m128 | xmm reg |
| m128 | m128 |

DO NOT support LOCK

This instruction is available in 64-bit mode and 32-bit mode. It requires the SSE feature set to be supported by the processor.

To avoid performance penalties or general protection faults, ensure the memory operand is correctly sized as m128. Unlike MOVAPS, MOVUPD DOES NOT require the memory address to be 16-byte aligned; however, accessing memory that crosses a page boundary MAY incur a performance penalty.
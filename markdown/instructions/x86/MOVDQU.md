> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# MOVDQU

Moves 128 bits of data from the source to the destination. Unlike MOVDQA, this instruction does not require the memory operand to be aligned on a 16-byte boundary.

The following table covers the supported source and destinations:

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |
| m128 | xmm |
| xmm | m128 |

DO NOT support LOCK

This instruction is available in 32-bit and 64-bit modes. In x86-64, it operates on XMM registers. It SHALL NOT be used in compatibility mode if the SSE feature set is not enabled in the target processor.

To avoid performance degradation, users SHOULD prefer MOVDQA when the data is known to be 16-byte aligned, as unaligned memory accesses MAY incur a penalty depending on the microarchitecture. If the memory operand crosses a cache line boundary, it SHALL result in multiple memory accesses.
> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VINSERTI128

Inserts a 128-bit lane from the source operand into the destination operand at the index specified by the immediate value.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| xmm reg | xmm reg |
| xmm reg | ymm reg |
| xmm reg | zmm reg |

DO NOT support LOCK

This instruction is available in 64-bit mode and compatibility mode. It requires the SSE4.1 extension for xmm destinations and AVX for ymm/zmm destinations.

The immediate value MUST be 0 or 1 when the destination is a ymm register, and MUST be 0, 1, 2, or 3 when the destination is a zmm register. Providing an immediate value outside of these ranges results in an #I. When operating on a ymm or zmm destination, the upper bits of the destination register not targeted by the insert operation are preserved.
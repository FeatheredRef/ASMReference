> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PUNPCKLDQ

Unpacks low quadwords from two 128-bit XMM registers. The low quadword from the first source operand and the low quadword from the second source operand are interleaved into the destination operand; specifically, the low quadword of the first source becomes the low quadword of the destination, and the low quadword of the second source becomes the high quadword of the destination.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |
| m16 | xmm |

DO NOT support LOCK

This instruction is available in 64-bit mode and compatibility mode. It requires SSE2 support.

To avoid data loss, the user SHALL be aware that the high quadwords of both source operands are discarded during this operation. This instruction is specifically designed for interleaving data; if the high quadwords contain necessary data, they MUST be processed via a separate `PUNPCKHDQ` instruction.
> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PUNPCKLQDQ

Unpacks quadwords from two XMM registers. The instruction selects the low quadword from the first source operand and the low quadword from the second source operand, and places them into the destination register.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |
| m128 | xmm |

DO NOT support LOCK

This instruction REQUIRES SSE4.1 support. It operates exclusively on 128-bit XMM registers and does not support operation in 64-bit mode via GPRs; it is strictly an SSE SIMD instruction.

To avoid unexpected behavior, note that the high quadwords of both source operands are discarded. If the destination register is also used as a source operand, the original high quadword of that register will be overwritten by the low quadword of the second source operand.
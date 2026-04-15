> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# KUNPCKBW

Unpacks words from two XMM registers into a single XMM register. For each pair of words from the source operands, the word from the first source is placed in the destination, followed by the word from the second source.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |
| #I | #I |

DO NOT support LOCK

This instruction is only available when SSE3 is supported. It operates exclusively on XMM registers; it SHALL NOT be used with YMM or ZMM registers.

To avoid data corruption, ensure that the source registers are properly initialized, as the instruction reads 128 bits from each source. Failure to use the correct register size in a 32-bit or 64-bit environment may lead to unexpected behavior if the upper bits of the register are not managed.
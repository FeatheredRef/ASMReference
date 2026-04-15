> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# ANDPD

Performs a bitwise logical AND operation on the lower 64 bits of two XMM registers. The result is stored in the destination register.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |
| #I | #I |

DO NOT support LOCK

This instruction is available in 64-bit mode and compatibility mode. It requires the SSE extension.

The operation only affects the lower 64 bits (quad word) of the XMM registers. The upper 64 bits of the destination register remain unchanged. Failure to account for the preserved upper bits may lead to logic errors when performing subsequent 128-bit operations.
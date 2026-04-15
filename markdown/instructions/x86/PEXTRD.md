> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PEXTRD

Extracts a dword from an XMM register based on an immediate index and stores it into a general-purpose register.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| xmm | r32 |

DO NOT support LOCK

This instruction is available in 64-bit mode and 32-bit mode. It requires the SSE4.1 instruction set extension.

The immediate index SHALL be an unsigned value between 0 and 3. If the index is out of this range, the operation is invalid. The result is zero-extended to the size of the destination register if the destination is r64.
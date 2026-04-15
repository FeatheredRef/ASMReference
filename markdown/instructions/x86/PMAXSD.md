> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PMAXSD

Compares the signed 32-bit integers in two XMM register elements and stores the maximum value for each corresponding element in the destination.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |

DO NOT support LOCK

This instruction is available in 64-bit mode and compatibility mode. It requires SSE4.1 support.

To avoid incorrect results, ensure that the operands are treated as signed i32 integers. If unsigned comparison is required, the `PMAXUD` instruction SHALL be used instead. This instruction does not modify the EFLAGS register.
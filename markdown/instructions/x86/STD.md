> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# STD

Stores the current floating-point precision control word into the specified memory location.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| internal register | m2 |

DO NOT support LOCK

The `STD` instruction is only available in 32-bit protected mode and compatibility mode. It is not supported in 64-bit mode.

The destination memory operand MUST be a word (2 bytes). Attempting to use a different operand size will result in an invalid operation. This instruction affects only the x87 FPU state and does not impact general-purpose registers.
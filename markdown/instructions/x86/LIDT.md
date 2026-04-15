> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# LIDT

Loads the Interrupt Descriptor Table register (IDTR) with a value from the specified memory location.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| m6 | IDTR |

DO NOT support LOCK

LIDT is available in 64-bit mode, compatibility mode, and 32-bit protected mode. The instruction requires a memory operand of 6 bytes; specifically, the first 2 bytes represent the limit of the IDT and the remaining 4 bytes represent the base address.

In 64-bit mode, the base address is expanded to 64 bits internally, but the LIDT instruction still loads from a 6-byte memory structure (where the base is provided as a 32-bit value). To load a full 64-bit base address, the `LIDT` instruction must be used in conjunction with a structure that the processor interprets according to the current mode; however, the standard `LIDT` instruction expects a 48-bit base in 64-bit mode (using a 10-byte structure) to avoid truncation. Using a 6-byte structure in 64-bit mode SHALL result in the upper 32 bits of the IDTR base being zeroed.
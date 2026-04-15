> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# LGDT

Loads the Global Descriptor Table register (GDTR) with a 6-byte value from the specified memory location.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| m6 | GDTR |

DO NOT support LOCK

The instruction is available in 16-bit, 32-bit, and 64-bit modes. In x86-64 architecture, when operating in 64-bit mode, the GDTR is loaded with a 10-byte value if the operand is treated as a 64-bit pointer, but the standard `LGDT` instruction specifically targets the 6-byte structure (16-bit limit and 32/64-bit base).

The source memory operand MUST be aligned to avoid potential performance penalties or faults depending on the processor configuration. Since the instruction loads the base address and limit of the GDT, the memory region m6 MUST contain a valid GDT descriptor; providing an incorrect address will result in General Protection faults (#GP) upon the next attempt to access a segment descriptor.
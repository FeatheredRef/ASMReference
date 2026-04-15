> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# LSS

Loads a segment selector and its descriptor's limit into a segment register and the TFSR (Task Register) or a specified memory location, typically used to load the Local Descriptor Table (LDT) selector into the LDTR.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| m2 | reg / m2 |

DO NOT support LOCK

The `LSS` instruction is NOT supported in 64-bit mode. It is only available when the processor is operating in compatibility mode or in legacy protected mode.

The instruction requires the source memory operand to be aligned. Accessing an unaligned memory location may result in a general-protection exception (#GP) depending on the alignment check (AC) flag in the EFLAGS register. Attempting to load an invalid selector or a selector that does not point to a valid LDT descriptor SHALL trigger a general-protection exception (#GP).
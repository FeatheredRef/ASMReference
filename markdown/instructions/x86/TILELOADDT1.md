> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# TILELOADDT1

Loads a 32-bit doubleword element from a linear address into a specified tile register.

The following table describes the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| m4 | tile register |

DO NOT support LOCK

This instruction is only available when the processor is operating in 64-bit mode and requires the AMX (Advanced Matrix Extensions) feature to be enabled in the BIOS and activated via the `XCR0` register.

The memory address must be aligned to the element size to avoid performance penalties; however, alignment requirements are governed by the tile configuration set by `TDPALIGN`. Failure to properly initialize the tile configuration using `TCONFIG` prior to execution will result in undefined behavior.
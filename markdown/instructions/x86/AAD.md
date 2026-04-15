> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# AAD

ASCII Adjust for Division converts the ASCII value in the `AL` register into a binary-coded decimal (BCD) value for use in division operations. It checks the value in `AL` and, if the lower nibble is greater than 9 or the upper nibble is non-zero, it adds 6 to `AL` and sets the Carry Flag (CF) to 1. If the conditions are not met, the Carry Flag is cleared.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg (AL) | reg (AL) |

DO NOT support LOCK

The `AAD` instruction is only available in 32-bit mode and 16-bit mode. In x86-64 architecture, it is ONLY supported when the processor is operating in compatibility mode. It is NOT supported in 64-bit mode.

To avoid incorrect calculations, `AAD` MUST be executed immediately before a `DIV` or `IDIV` instruction to ensure the dividend is properly adjusted from ASCII to BCD format. Failure to do so will result in the division of an ASCII representation rather than the intended numeric value.
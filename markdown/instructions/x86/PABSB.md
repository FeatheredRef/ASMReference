> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PABSB

PABSB (Page Attribute Table Select Byte) updates the Page Attribute Table (PAT) by selecting a byte from a specified memory location and loading it into the PAT register.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| m1 | PAT Register |
| reg | #I |
| imm | #I |

DO NOT support LOCK

This instruction is ONLY available in 64-bit mode. It is NOT supported in compatibility mode or 32-bit protected mode.

To avoid unexpected memory typing behavior, the source address MUST be aligned to the architectural requirements of the memory type being accessed. Failure to provide a valid memory operand will result in an invalid operation.
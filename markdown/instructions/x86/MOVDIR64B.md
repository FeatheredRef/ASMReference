> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# MOVDIR64B

Copies a 64-bit doubleword from the source operand to the destination operand using direct store technology, bypassing the cache hierarchy to write directly to system memory.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| r64 | m8 |
| m8 | #I |
| imm | #I |

DO NOT support LOCK

This instruction SHALL only be executed when the processor is in 64-bit mode. It is NOT supported in compatibility mode.

The destination operand MUST be aligned on an 8-byte boundary. If the destination is not 8-byte aligned, a general-protection exception (#GP) SHALL be generated. This instruction requires the processor to support the Direct Store feature; if not supported, executing this instruction results in an undefined operation (#UD).
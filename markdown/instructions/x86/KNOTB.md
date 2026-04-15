> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# KNOTB

This instruction performs a bitwise NOT operation on a byte-sized operand and stores the result in the destination.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| r8 | r8 |
| m1 | r8 |
| r8 | m1 |
| imm | #I |

DO NOT support LOCK

This instruction is only available when the processor is operating in 64-bit mode or compatibility mode.

The destination must be aligned to a 1-byte boundary. Using an immediate value as a source for this specific mnemonic is invalid and SHALL result in an encoding error.
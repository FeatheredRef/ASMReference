> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# MOVQ

Copies a quad word operand from the source to the destination.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| r64 | r64 |
| r64 | m8 |
| imm | r64 |
| m8 | r64 |
| m8 | m8 |

DO NOT support LOCK

This instruction is available in 64-bit mode and compatibility mode. In 32-bit mode, the `MOVQ` mnemonic is typically used for MMX register moves, which behave differently than the General Purpose Register (GPR) moves described here.

When moving an immediate to a register, the immediate MUST be sign-extended to 64 bits. Users MUST ensure that the destination memory address is aligned to a 8-byte boundary to avoid performance penalties or alignment faults in strict environments. Attempting to use `MOVQ` with an immediate source and a memory destination is #I; a register MUST be used as an intermediary.
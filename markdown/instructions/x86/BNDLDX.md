> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# BNDLDX

Loads the bound register BND64 (or BND32) from the specified memory location into the destination register.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| m8 | r8 |
| m32 | r32 |
| m64 | r64 |

DO NOT support LOCK

This instruction is only available when the processor is operating in 64-bit mode or compatibility mode. It is part of the MPX (Memory Protection Extensions) feature set; if the processor does not support MPX or if MPX is disabled in the BIOS/Operating System, executing this instruction will result in an invalid opcode exception.

Ensure that the memory operand is properly aligned to the size of the bound register being loaded to avoid performance penalties or alignment faults. The instruction loads the bound value into the BND register, which is used by `BNDCL` and `BNDCU` for range checking; failure to initialize these registers via `BNDLD` before checking will result in undefined boundary checks.
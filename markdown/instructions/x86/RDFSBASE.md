> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# RDFSBASE

Reads the current base address of the `%fs` segment and stores it into the specified destination register.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| Internal FS Base | r64 |

DO NOT support LOCK

This instruction is only available when the processor is operating in 64-bit mode. It is NOT supported in compatibility mode.

The instruction ignores the current value of the FS segment selector in the segment register; it retrieves the base address directly from the hidden portion of the descriptor cache (the base address specified in the Model Specific Register `IA32_FS_BASE`). If the processor is not in 64-bit mode, executing this instruction SHALL result in an invalid opcode exception.
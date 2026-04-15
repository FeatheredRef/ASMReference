> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# FNSAVE

Saves the current x87 FPU environment—including the register stack, control word, status word, tag word, and other state information—to a specified memory location.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| Internal FPU State | m108 |

DO NOT support LOCK

This instruction is available in 32-bit mode and 64-bit mode (compatibility mode). In 64-bit mode, it operates only when the processor is executing in compatibility mode.

The memory destination `m108` MUST be 16-byte aligned to avoid potential performance penalties or alignment exceptions depending on the environment. Failure to provide a valid memory address will trigger a general-protection exception (#GP). The instruction does not trigger any floating-point exceptions (#I, #Z, #D, #O, #U, #P).
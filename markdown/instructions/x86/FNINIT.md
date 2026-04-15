> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# FNINIT

Initializes the FPU by resetting the FPU control word, status word, and tag word to their default values.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| #I | #I |

DO NOT support LOCK

FNINIT is available in 64-bit mode, though the x87 FPU is primarily used for compatibility. It does not require any operands.

The instruction does not trigger any floating-point exceptions (#I, #Z, #D, #O, #U, #P). Since it resets the status word, any previously pending floating-point exceptions are cleared.
> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# FCLEX

Clears the floating-point exception control word and the floating-point exception status word.

The following table covers the supported source and destinations:

| source | destination(s) |
| :--- | :--- |
| #I | FP Status/Control Word |

DO NOT support LOCK

This instruction is available in both 64-bit mode and compatibility mode. It specifically targets the x87 FPU state regardless of the current execution mode.

To avoid unexpected behavior in multitasking environments, ensure the FPU state is saved via `FSAVE` or `FXSAVE` before executing `FCLEX` if the previous status flags are required for error handling.
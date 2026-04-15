> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# FLDCW

Loads the x87 floating-point control word from a specified memory location into the x87 control register.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| mword | x87 Control Register |

DO NOT support LOCK

This instruction is only available in x86-64 compatibility mode. It is not supported in x86-64 64-bit mode.

The memory operand MUST be aligned to a word boundary to avoid potential performance penalties or alignment exceptions depending on the processor configuration. Writing an invalid value to the control word may result in unexpected floating-point precision or rounding behavior for subsequent x87 instructions.
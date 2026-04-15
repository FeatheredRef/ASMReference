> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# FLDLG2

Loads the 64-bit floating-point constant (the base-2 logarithm of 2) onto the ST(0) register of the x87 FPU stack.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| imm | #I |
| reg | #I |
| memory | #I |

DO NOT support LOCK

This instruction is available in 64-bit mode, 32-bit mode, and compatibility mode. It does not require a source operand as the value is an implicit architectural constant.

The instruction increments the FPU stack pointer (TOP). If the stack pointer is already at the maximum value (4), a stack overflow exception (#SS) SHALL occur. The precision of the loaded constant depends on the current FPU control word setting.
> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# FICOM

Compares a floating-point value stored in the ST(0) register with a floating-point value from a specified source. The instruction compares the two values and sets the Floating-Point Condition Codes (FPCond) based on whether ST(0) is greater than, less than, or equal to the source operand.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg (ST(n)) | ST(0) |
| m32/m64 | ST(0) |

DO NOT support LOCK

This instruction is only available in compatibility mode when executing 32-bit code or in 64-bit mode via the x87 FPU stack. It is not supported in native x86-64 mode if the FPU state is not properly initialized.

The source operand must be a valid floating-point format (f32 or f64). If the operand is a memory location (m32/m64), the size of the memory operand must match the current precision of the FPU environment. An invalid operand format will trigger #I. If either operand is a signaling NaN, #I is generated; if both are quiet NaNs, the result is undefined and may trigger #P. Precision exceptions (#P) and underflow exceptions (#U) may occur during the internal conversion of operands to the 80-bit extended precision format.
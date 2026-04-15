> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# FLD

Pushes a floating-point value from the specified source onto the floating-point register stack.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg (ST(i)) | ST(0) |
| m4 (f32) | ST(0) |
| m8 (f64) | ST(0) |
| m10 (f80) | ST(0) |

DO NOT support LOCK

This instruction is supported in 64-bit mode, though the x87 FPU is primarily maintained for compatibility. The behavior is subject to the current floating-point control word.

If the floating-point stack is full, this instruction SHALL trigger a stack overflow exception. When loading from memory, the size of the operand (f32, f64, or f80) determines the precision of the value pushed; f32 and f64 values are converted to f80 internally. Precision control settings in the FPU control word may cause #P if the value cannot be represented exactly in the target precision.
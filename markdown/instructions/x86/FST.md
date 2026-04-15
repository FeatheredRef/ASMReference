> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# FST

Stores the ST(0) floating-point value from the FPU register stack to the specified destination. The value is rounded according to the current rounding control in the FPU control word.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| ST(0) | m32 |
| ST(0) | m64 |
| ST(0) | r32 |
| ST(0) | r64 |

DO NOT support LOCK

This instruction is available in 32-bit mode and 64-bit mode. In 64-bit mode, it is supported for compatibility with the x87 FPU architecture.

When storing to memory, the destination must be aligned to the size of the operand to avoid performance penalties. If the destination is a 64-bit memory location, the instruction stores the value as a double-precision floating-point number (f64). If the destination is a 32-bit memory location, the value is rounded to single-precision (f32) before storage.

Failure to account for the current rounding mode in the FPU control word MAY result in unexpected precision loss or #P (Inexact result) exceptions. The instruction will trigger #O (Numeric overflow) if the result cannot be represented in the destination format.
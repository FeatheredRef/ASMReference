> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# FADDP

Adds the floating-point value from a source to the value at the top of the floating-point register stack (ST(0)), pops the source from the stack if it was ST(0), and stores the result in ST(0).

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| ST(i) | ST(0) |
| m32 | ST(0) |
| m64 | ST(0) |
| m80 | ST(0) |

DO NOT support LOCK

This instruction is available in 64-bit mode, but it is part of the x87 floating-point instruction set. Use of x87 instructions in 64-bit mode is supported, although SSE/AVX are generally preferred for performance.

When using the `FADDP ST(i), ST(0)` form, the stack pointer is decremented by one after the operation. If the source is a memory operand (`m32`, `m64`, or `m80`), the stack pointer remains unchanged.

The operation may trigger the following floating-point exceptions:
- #I: If any operand is a signaling NaN.
- #Z: Not applicable for addition.
- #D: If an operand is denormal.
- #O: If the result is too large to be represented.
- #U: If the result is too small to be represented.
- #P: If the result is rounded.

The precision of the operation is determined by the current x87 control word. Failure to ensure the precision of memory operands matches the internal 80-bit representation may result in #P.
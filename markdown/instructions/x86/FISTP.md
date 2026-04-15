> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# FISTP

Stores an extended-precision floating-point value from the top of the ST(0) register to a memory location and pops the ST(0) register.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| f80 | m10 |

DO NOT support LOCK

This instruction is available in both 64-bit mode and compatibility mode. It operates exclusively on the x87 Floating-Point Unit (FPU) register stack.

The destination memory operand MUST be aligned to the appropriate boundary to avoid potential performance penalties or alignment exceptions depending on the processor configuration. The value is stored as an 80-bit extended-precision floating-point number. If the destination memory region is not exactly 10 bytes, it MAY result in an out-of-bounds memory access.

The instruction MAY trigger the following exceptions:
- #O: Numeric overflow.
- #U: Numeric underflow.
- #P: Inexact result (precision).
> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# FLD1

Pushes the floating-point constant 1.0 onto the top of the floating-point register stack.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| imm | ST(0) |

DO NOT support LOCK

This instruction is available in 64-bit mode, compatibility mode, and 32-bit mode. It operates exclusively on the x87 floating-point register stack.

The instruction increments the floating-point stack pointer (TOP). If the stack pointer is at the maximum value (7), this instruction SHALL trigger a stack overflow exception. The value 1.0 is loaded as an 80-bit extended precision floating-point number.
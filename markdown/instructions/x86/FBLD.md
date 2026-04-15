> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# FBLD

Loads a long real (80-bit extended precision) floating-point value from memory and pushes it onto the floating-point register stack.

The table below covers the supported source and destination.

| Source | Destination(s) |
| :--- | :--- |
| m10 | ST(0) |

DO NOT support LOCK

This instruction is available in 64-bit mode, but it specifically targets the x87 FPU stack. It is not available in compatibility mode if the x87 state is not preserved or enabled.

The operand MUST be a 10-byte extended precision floating-point value. Using an incorrect memory size or an unsupported address alignment may trigger a general protection fault (#GP). 

The instruction increments the floating-point stack pointer (TOP). If the stack is full (TOP=0), a stack overflow exception (#SF) occurs.
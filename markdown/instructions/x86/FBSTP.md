> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# FBSTP

Stores the ST(0) floating-point stack element to the specified memory location and then pops the ST(0) element.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| ST(0) | m32 / m64 |

DO NOT support LOCK

This instruction is supported in 64-bit mode, compatibility mode, and 32-bit mode. It operates exclusively on the x87 Floating-Point Unit (FPU) register stack.

The destination memory operand MUST be aligned according to the size of the floating-point value being stored to avoid potential performance penalties or alignment exceptions. If the memory operand is a m32, a 32-bit floating-point value is stored; if it is a m64, a 64-bit floating-point value is stored.

The instruction may trigger the following exceptions based on the value in ST(0):
- #Q: If the value in ST(0) is a signaling NaN.
- #U: If the value in ST(0) is an unsigned number and triggers an underflow.
- #D: If the value in ST(0) is a denormalized number.

Because this is a popping store, the stack pointer is decremented. Failure to account for the pop operation may lead to stack overflow if FBSTP is not used to balance corresponding FPU push instructions (e.g., FLD).
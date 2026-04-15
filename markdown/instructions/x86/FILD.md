> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# FILD

Loads a long real (64-bit) value from memory onto the ST(0) register of the FPU stack and increments the stack pointer.

The following table covers the supported source and destinations:

| source | destination(s) |
| :--- | :--- |
| m8 | ST(0) |

DO NOT support LOCK

This instruction is supported in 64-bit mode and compatibility mode. It operates exclusively on the x87 FPU register stack.

The memory operand MUST be a 64-bit floating-point value (long real). Providing a pointer to a smaller data size will result in the instruction reading an additional 4 bytes from the adjacent memory address, potentially causing a segmentation fault or loading corrupted data. 

The instruction may trigger the following exceptions:
- #S: If the FPU stack is full.
- #P: If the loaded value is rounded to fit the internal 80-bit precision.
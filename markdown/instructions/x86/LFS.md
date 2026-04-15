> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# LFS

Loads a scalar floating-point value from a memory location into a scalar floating-point register.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| m32 | f32 |
| m64 | f64 |

DO NOT support LOCK

This instruction is only available in compatibility mode. It operates on the x87 FPU stack and is not supported in native x86-64 long mode.

The instruction requires the memory operand to be naturally aligned to the size of the data being loaded to avoid performance penalties or alignment check exceptions if alignment checking is enabled. The value is pushed onto the x87 floating-point register stack; if the stack is full, a stack overflow exception occurs.
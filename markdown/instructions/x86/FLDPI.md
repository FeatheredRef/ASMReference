> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# FLDPI

Pushes the mathematical constant $\pi$ onto the floating-point register stack.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| imm | st(0) |

DO NOT support LOCK

This instruction is available in 64-bit mode, though it operates on the x87 FPU register stack. It is not available in the AVX-512 or SSE register sets.

The constant $\pi$ is loaded with 64-bit precision. If the current precision of the FPU is set to a value other than 64 bits, the value is rounded to the current precision, which MAY trigger #P.
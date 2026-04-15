> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# EMMS

Sets the FPU tag word to 0, effectively emptying the FPU register stack.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| #I | #I |

DO NOT support LOCK

The instruction is supported in 64-bit mode and compatibility mode. It specifically affects the x87 FSTK (Floating-Point Stack) state.

Failure to execute EMMS after using FPU instructions in a mixed-mode environment MAY lead to `FPU stack overflow` exceptions if the stack pointer (TOP) is not reset before subsequent FPU operations. This is critical when interfacing with legacy code or libraries that expect a clean FPU stack.
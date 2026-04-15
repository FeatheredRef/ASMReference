> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# FLDENV

Stores the contents of the current FPU environment—including the control word, status word, tag word, and the current floating-point instruction pointer—into a specified memory location.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| Internal FPU State | m28 |

DO NOT support LOCK

This instruction is supported in 64-bit mode and compatibility mode. It operates specifically on the x87 FPU state regardless of the current processor mode.

The destination memory region m28 MUST be aligned to a dword boundary to avoid potential performance penalties or alignment checks. Writing the FPU environment to memory is typically used before performing a context switch or when implementing exception handlers to preserve the floating-point state. Failure to allocate exactly 28 bytes of contiguous memory will result in a buffer overflow.
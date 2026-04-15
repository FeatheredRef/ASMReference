> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# FSTENV

Stores the x87 FPU control word and the current floating-point environment (including the last control word and the last operation's status) from the FPU to the specified memory location.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| Internal FPU State | m52 |

DO NOT support LOCK

This instruction is supported in 64-bit mode and compatibility mode. It is used to save the FPU environment for exception handling or state restoration.

The destination memory region m52 MUST be aligned to a dword boundary to avoid performance penalties or faults on certain architectures. Writing to a memory region that overlaps with the FPU stack or control registers may lead to undefined behavior. Since FSTENV stores a block of 52 bytes, the programmer SHALL ensure that the destination memory buffer is sufficiently large to prevent buffer overflows.
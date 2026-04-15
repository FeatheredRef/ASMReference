> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# FNSTENV

Stores the current FPU environment (including the control word, status word, tag word, and instruction pointer) to the specified memory location.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| Internal FPU State | m52 |

DO NOT support LOCK

This instruction is available in 64-bit mode and compatibility mode. It operates on the x87 FPU state; it does not affect SSE/AVX registers.

The destination memory region m52 MUST be aligned to a dword boundary to avoid performance penalties or faults on certain platforms. If the instruction is executed in 64-bit mode, the stored instruction pointer is 64 bits, and the memory layout conforms to the x86-64 specific FPU environment structure. Failure to provide a sufficient memory buffer of 52 bytes WILL result in memory corruption of adjacent data.
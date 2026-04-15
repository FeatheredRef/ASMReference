> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# FSAVE

Saves the current state of the x87 FPU register set (including the FPU stack, control word, status word, tag word, and other internal registers) to the specified memory location.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| Internal FPU State | m100 |

DO NOT support LOCK

FSAVE is available in 64-bit mode, but it is primarily used for legacy support. While it operates in 64-bit mode, the destination memory address must be aligned according to the architectural requirements of the target environment to avoid performance penalties.

The destination memory region m100 SHALL be at least 100 bytes in size. Writing to a buffer smaller than 100 bytes SHALL result in a memory overwrite of adjacent data. FSAVE does not save the FPU instruction pointer or the state of the SSE/AVX registers; only the x87 FPU state is preserved.
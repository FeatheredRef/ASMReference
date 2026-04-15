> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# FXSAVE

Saves the entire FPU state (x87 FPU registers, x87 control word, status word, tag word, and FPU instruction pointer) to a memory location.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| Internal FPU State | m512 |

DO NOT support LOCK

The destination m512 SHALL be 16-byte aligned. If the destination is not 16-byte aligned, the processor generates a general-protection exception (#GP). This instruction is supported in 64-bit mode and compatibility mode.

To avoid #GP exceptions, the programmer MUST ensure the memory operand is aligned to a 16-byte boundary. The state saved by FXSAVE is not compatible with the legacy FSAVE instruction; the corresponding restore instruction SHALL be FXRSTOR.
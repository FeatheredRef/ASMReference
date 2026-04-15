> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VGATHERDPS

Gathers packed single-precision floating-point values from non-contiguous memory locations into a destination register. The instruction uses a base address and a set of indices provided in a register to calculate the effective addresses of the elements to be loaded.

The table below covers the supported sources and destinations.

| source | destination(s) |
| :--- | :--- |
| m32 | r128/r256/r512 |
| r128/r256/r512 | r128/r256/r512 |

DO NOT support LOCK

This instruction is available only in 64-bit mode and 32-bit mode. It REQUIRES support for the AVX2 instruction set.

The instruction uses a mask register to track which elements have been successfully gathered. If a mask bit is set to 0, the corresponding element is skipped. Elements are processed sequentially; if a memory fault occurs, the mask is updated to reflect completed loads, allowing the instruction to be resumed after the exception is handled.

To avoid performance degradation or unexpected behavior, ensure that the index register contains valid offsets relative to the base address. Memory accesses that cross page boundaries may result in significant performance penalties. The destination register is not fully overwritten if the mask is not fully set; existing values in the destination for masked-out elements are preserved.
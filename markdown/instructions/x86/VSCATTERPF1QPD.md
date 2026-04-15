> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VSCATTERPF1QPD

VSCATTERPF1QPD scatters 64-bit floating-point values from a ZMM register to memory locations specified by a vector of indices in another ZMM register, using a specified scale. Each value is written to memory if the corresponding mask bit is set.

The following table describes the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| zmm | m64 |
| zmm | m64 |
| imm | #I |

DO NOT support LOCK

This instruction is available only in 64-bit mode. It requires the AVX-512PF (Prefetch) instruction set extension.

The instruction MUST be used with a mask register; if the mask bit for a specific element is 0, the corresponding memory write is suppressed. To avoid memory faults, the index vector MUST contain valid addresses within the accessible address space. The instruction does NOT guarantee the order of stores; if multiple indices point to the same memory location, the final value stored is non-deterministic. Overlapping memory regions may result in undefined behavior.
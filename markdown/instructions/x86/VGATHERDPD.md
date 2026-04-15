> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VGATHERDPD

Gathers 64-bit floating-point values from memory using a base address and a set of 32-bit signed indices. For each index in the index register, the instruction calculates the effective address by adding the scaled index to the base address, then loads the 64-bit value from that address into the destination register. A mask register tracks which elements have been successfully loaded; if an element is loaded, the corresponding mask bit is cleared.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| r64, zmm/ymm | zmm/ymm |
| m64 | zmm/ymm |

DO NOT support LOCK

This instruction SHALL ONLY be executed in 64-bit mode. It REQUIRES AVX2 support.

Faults occurring during the gather operation are handled such that the instruction is restartable; the mask register is updated to reflect which elements were successfully processed before the fault occurred. Memory accesses SHALL be performed using the current paging attributes.

To avoid performance degradation or unexpected behavior, ensure that the mask register is properly initialized, as elements with a mask bit set to 0 are skipped. If the base address or indices cause an out-of-bounds memory access, a general-protection fault (#GP) or page fault (#PF) may occur depending on the memory region. The instruction is NOT atomic.
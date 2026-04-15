> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VGATHERPF0QPS

Gathers packed single-precision floating-point values from non-contiguous memory locations into a destination register. It uses a base address, a vector of indices, a scale factor, and a mask register. For each active element in the mask, the instruction calculates an effective address by adding the base address to the product of the index and scale, fetches the `f32` value from memory, and stores it in the destination. The mask is then updated to clear the bits corresponding to successfully loaded elements.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| reg (base), reg (indices), m32/reg (mask) | reg (dest), reg (mask) |

DO NOT support LOCK

This instruction is available only in 64-bit mode. It requires support for the AVX2 instruction set.

The mask register is updated upon completion; if a memory access fails (e.g., due to a page fault), the instruction must be restartable. The mask bit must be set to 1 for the element to be processed. If the mask bit is 0, the corresponding element in the destination register remains unchanged.

To avoid performance degradation or unexpected behavior, ensure that the index values do not cause the effective address to wrap around the address space. Memory access is subject to alignment constraints; while the instruction supports unaligned access, aligned accesses are generally more performant. If a fault occurs during the gather operation, the mask register is updated such that only the elements that were not yet successfully loaded remain marked for processing.
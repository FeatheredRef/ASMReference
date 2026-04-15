> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VPGATHERQD (1)

Collects 64-bit signed integers from memory using a base address and a vector of 64-bit signed indices. For each element, the instruction calculates the effective address by adding the base address to the index multiplied by the scale, then loads the qword from that address into the destination vector. A mask register controls which elements are gathered; elements with a mask bit of 0 are skipped, and the corresponding destination elements remain unchanged.

The following table describes the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| m8 | reg |

DO NOT support LOCK

This instruction SHALL only be executed in 64-bit mode. It REQUIRES AVX2 support.

The mask register is updated during execution; if a load fails or is skipped, the corresponding mask bit is cleared. Memory accesses are not guaranteed to be in any specific order. If a page fault occurs during the gather operation, the instruction SHALL be restartable; the mask register ensures that elements already loaded are not re-processed.

To avoid undefined behavior or crashes, ensure that the base address and indices do not result in an effective address outside the accessible canonical address space. Since this instruction performs multiple memory accesses, it can trigger multiple page faults. Use a valid mask to prevent accessing invalid memory locations.
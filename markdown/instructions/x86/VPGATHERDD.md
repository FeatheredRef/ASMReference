> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VPGATHERDD

Gathers 32-bit signed integers from memory using a base address and a set of 32-bit signed indices provided in a vector register. For each index, the instruction calculates an effective address by adding the scaled index to the base address, fetches the dword from that address, and stores it in the destination vector register. The mask register determines which elements are gathered; elements with a mask bit set to 0 are skipped, and the corresponding mask bit is cleared upon successful completion of the load.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| reg, m32 | zmm/ymm/xmm |

DO NOT support LOCK

The instruction SHALL be executed in 64-bit mode or 32-bit mode. It REQUIRES AVX2 and AVX-512 support. The behavior of the instruction is constrained by the mask register; if the mask is all zeros, the instruction performs no memory accesses.

To avoid faults, ensure that the calculated effective addresses are within the accessible page boundaries. If a gather operation encounters a memory fault for an element whose mask bit is set, the instruction SHALL trigger a general-protection fault or page fault. To prevent performance degradation, avoid accessing memory across page boundaries where possible. Note that the order of memory accesses is not guaranteed to be sequential.
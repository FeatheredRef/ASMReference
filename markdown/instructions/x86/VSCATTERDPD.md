> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VSCATTERDPD

VSCATTERDPD scatters 64-bit floating-point values from a ZMM register to memory locations specified by a vector of 64-bit indices. It uses a mask register to determine which elements are written to memory; only elements corresponding to a set bit in the mask are stored.

The following table describes the supported source and destination operands:

| source | destination(s) |
| :--- | :--- |
| zmm / m512 | m64 |

DO NOT support LOCK

This instruction is only available in 64-bit mode. It requires the AVX-512 foundation and the AVX-512DQ instruction set.

Faults may occur if the calculated memory address is invalid. If the mask bit for a specific element is 0, no memory access is performed for that element, and any potential faults associated with that address are suppressed. To avoid undefined behavior or memory corruption, the programmer SHALL ensure that the indices in the index vector do not cause the calculated address to wrap around the address space. Memory ordering is not guaranteed for the scatter operation; the order of stores depends on the hardware implementation.
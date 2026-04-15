> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VSCATTERPF0DPS

This instruction scatters packed single-precision floating-point values from a zmm register to memory locations specified by an index vector. It uses a base address, an index vector of 32-bit signed integers, and a scale factor to calculate the destination addresses for each element.

The following table specifies the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| zmm | mN |
| zmm | mN |
| zmm | mN |

DO NOT support LOCK

This instruction is available only in 64-bit mode. It requires the AVX-512 foundation and the AVX-512PF (Prefetch) extension.

To avoid unexpected behavior, ensure that the index vector does not contain values that result in memory addresses outside the accessible address space. Masking should be used to prevent illegal memory accesses. Memory must be aligned according to the requirements of the data type being stored to avoid performance degradation or faults.
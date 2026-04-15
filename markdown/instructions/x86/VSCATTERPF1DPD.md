> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VSCATTERPF1DPD

VSCATTERPF1DPD scatters double-precision floating-point values from a zmm register to memory locations based on indices provided by another zmm register. For each active element indicated by the mask, the instruction writes a 64-bit floating-point value to the address calculated by adding the scaled index to the base address.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| zmm (indices) | m64 |
| zmm (values) | m64 |
| k (mask) | m64 |

DO NOT support LOCK

This instruction SHALL only be executed in 64-bit mode. It is not supported in compatibility mode.

The instruction requires the AVX-512 foundation and the AVX-512PF (Prefetch) extension. If the processor does not support AVX-512PF, executing this instruction SHALL trigger an invalid opcode exception.

The behavior of VSCATTERPF1DPD is non-deterministic regarding the order of memory writes. If multiple indices point to the same memory location, the final value written is not guaranteed. Users MUST ensure that the index register does not contain overlapping addresses if the sequence of writes is critical to the application logic. Masking SHALL be used to prevent invalid memory accesses; any element with a mask bit set to 0 is ignored and no memory operation is performed for that index.
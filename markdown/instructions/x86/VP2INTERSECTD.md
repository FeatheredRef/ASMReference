> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VP2INTERSECTD

Performs a bitwise intersection operation on two 512-bit zmm registers, treating them as vectors of dword elements. For each element, it computes the intersection of the bit patterns, typically used in specific AVX-512 extensions to identify common set bits between two masks or data vectors.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| reg | reg |

DO NOT support LOCK

This instruction is available only in 64-bit mode. It requires AVX-512 support and the corresponding CPUID feature flag. 

The instruction is subject to alignment constraints; while unaligned memory access is supported by the ISA, accessing memory not aligned to 64 bytes may result in performance degradation. This instruction does not modify any status flags in the EFLAGS register. When used with opmask registers (k-registers), the operation is performed only on the elements enabled by the mask; elements not enabled are zeroed or left unchanged depending on the masking flavor (merging vs. zeroing).
> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VPSCATTERQQ

Scatters 64-bit quadword elements from a ZMM register to memory locations specified by an index vector in a ZMM register, using a base address and a scale factor.

The following table specifies the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| reg | m64 |
| reg | reg |

DO NOT support LOCK

This instruction is available only in 64-bit mode. It requires AVX-512 support and specifically the AVX-512 Foundation (F) instruction set.

The instruction MUST utilize a mask register (k-register) to determine which elements are scattered. If a mask bit is 0, the corresponding element is NOT written to memory.

Memory accesses are performed using the formula: `Address = Base + (Index * Scale)`. If the index is out of bounds or the memory access violates segmentation or paging rules, a general protection fault or page fault MAY occur depending on the mask bit for that specific element.

Write-ordering is NOT guaranteed for elements within the same instruction; therefore, if multiple indices in the index vector point to the same memory location, the final value written is non-deterministic.
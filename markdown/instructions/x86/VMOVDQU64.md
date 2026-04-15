> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VMOVDQU64

Moves 64 bits of data from the source to the destination. The instruction performs an unaligned move, meaning the memory address does not need to be aligned to a 8-byte boundary.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| r64 | r64 |
| r64 | m8 |
| m8 | r64 |

DO NOT support LOCK

This instruction is available in 64-bit mode and compatibility mode. It is part of the AVX extension set; therefore, the CPU MUST support the AVX ISA for this instruction to be executable.

To avoid performance degradation or general protection faults, ensure that the memory operand does not cross a page boundary unless the hardware supports unaligned accesses across pages. While the instruction is designed for unaligned memory, accessing memory outside the allocated segment or page boundaries will trigger a page fault.
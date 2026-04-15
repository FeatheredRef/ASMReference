> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# INVLPG

Invalidates the translation lookaside buffer (TLB) entry for a specific virtual address.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| m8/m16/m32/m64 | #I |

DO NOT support LOCK

The instruction is available in 64-bit mode and compatibility mode. It REQUIRES that the address be a valid linear address. The instruction does not affect the TLB entries of other processors; therefore, in a multiprocessor system, a "TLB shootdown" MUST be implemented via software to invalidate entries across all cores.

The operand MUST be a memory address; providing an immediate or register directly as the operand is not supported by the opcode. If the address is not present in the page tables, the instruction executes without effect. Failure to invalidate the TLB after modifying a page table entry MAY lead to the processor using stale translation data, resulting in unpredictable memory access or protection faults.
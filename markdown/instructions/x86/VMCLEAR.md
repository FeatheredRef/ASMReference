> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VMCLEAR

VMCLEAR clears the Virtual Machine Control Structure (VMCS) pointed to by the VMCS pointer, effectively invalidating the VMCS and removing it from the processor's internal cache.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| m64 | #I |

DO NOT support LOCK

VMCLEAR SHALL only be executed in VMX root operation. If executed outside of VMX root operation, the instruction SHALL trigger a general protection fault (#GP).

Ensure that the memory operand points to a valid physical address. If the address is not aligned to a 4-byte boundary, the instruction SHALL trigger a #GP. Failure to properly clear a VMCS before reusing the memory region for a different purpose may result in undefined behavior during subsequent VMPTRLD operations.
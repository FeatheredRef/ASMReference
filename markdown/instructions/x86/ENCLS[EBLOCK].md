> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# ENCLS[EBLOCK]

Encloses the memory region specified by the operand within an Enclave Page Cache (EPC) block, marking the page as part of an enclave's private memory space and ensuring that the memory is protected from access by software outside the enclave.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| imm | #I |
| reg | #I |
| m8 | #I |
| m32 | #I |
| m64 | #I |
| qword | #I |

DO NOT support LOCK

This instruction is only available when the processor is executing in Long Mode and is specifically intended for use within Intel SGX (Software Guard Extensions) enabled environments. It SHALL NOT be executed in compatibility mode.

The instruction requires that the target memory address be aligned to a page boundary. Failure to provide a properly aligned address SHALL result in a general-protection exception (#GP). Additionally, the instruction requires that the target page be currently mapped as unassigned within the EPC; if the page is already associated with an enclave or is not a valid EPC page, the operation SHALL fail.
> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# ENCLS[EREMOVE]

Removes a page from the enclave's Page Cache (EPC), effectively removing the mapping of the specified linear address from the enclave.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg | #I |

DO NOT support LOCK

This instruction is ONLY available when the processor is executing in Enclave Mode. Execution of ENCLS outside of an enclave SHALL result in a general protection fault (#GP).

The linear address to be removed MUST be provided in the `rax` register. If the address is not valid or not mapped within the enclave, the instruction SHALL return an error code in `rax` without modifying the page cache. Failure to verify the return value in `rax` may lead to inconsistent memory views within the enclave.
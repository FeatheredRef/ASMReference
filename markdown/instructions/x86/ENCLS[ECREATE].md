> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# ENCLS[ECREATE]

The `ENCLS[ECREATE]` instruction creates a new enclave by initializing the enclave's metadata and allocating the necessary resources within the Processor Reserved Memory (PRM). It validates the enclave's configuration and sets up the enclave's memory layout based on the provided parameters.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| m8 | #I |
| reg | #I |
| imm | #I |

DO NOT support LOCK

This instruction is only available when the processor is operating in long mode and is specifically designed for use with Intel SGX (Software Guard Extensions). It SHALL only be executed from outside an enclave (non-enclave mode).

To avoid initialization failures, the software MUST ensure that the memory region pointed to by the enclave's configuration is properly aligned and that the Page Attribute Table (PAT) is configured correctly. Failure to provide a valid ECREATE structure in memory will result in an error code being returned in the destination register rather than throwing a general protection fault.
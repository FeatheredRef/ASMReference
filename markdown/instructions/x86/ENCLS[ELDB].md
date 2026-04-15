> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# ENCLS[ELDB]

Loads a data value from a memory location into a register, specifically used within the Enclave Page Cache (EPC) to retrieve encrypted data.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| mN | rN |

DO NOT support LOCK

This instruction is only available when the processor is operating in CPL 3 and is specifically designed for use within Intel SGX enclaves. It SHALL only be executed inside an enclave; executing this instruction outside of an enclave context results in a general protection fault (#GP).

The memory operand MUST be a valid address within the Enclave Page Cache (EPC). If the memory access violates the enclave's access control or the address is not mapped to the EPC, the instruction SHALL trigger a page fault or a general protection fault. Failure to ensure the memory operand is aligned to the size of the data being loaded MAY result in alignment check exceptions depending on the CR0.AC setting.
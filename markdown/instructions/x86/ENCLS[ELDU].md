> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# ENCLS[ELDU]

Loads the Enclave Page Cache Map (EPCM) entry associated with a linear address into a register.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| m8 | r64 |

DO NOT support LOCK

This instruction is only available when the processor is executing within an enclave (Enclave Mode). Attempting to execute this instruction outside of an enclave SHALL result in a general protection fault (#GP(0)).

The linear address provided must be a valid address within the enclave's page cache. If the address is not valid or the processor lacks the required permissions to access the EPCM entry, the instruction SHALL fail and set the appropriate error code in the destination register. Users MUST verify the return value to ensure the EPCM entry was successfully loaded before relying on the data.
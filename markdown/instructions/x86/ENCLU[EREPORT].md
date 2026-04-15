> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# ENCLU[EREPORT]

The `ENCLU[EREPORT]` instruction is used within an Intel SGX enclave to generate a report that provides evidence of the enclave's identity, attributes, and current state. It fills a specified memory region with an `EREPORT` structure, which includes a measurement of the enclave and a MAC (Message Authentication Code) for verification by a quoting enclave or an external party.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| m8 | #I |
| reg | #I |
| imm | #I |
| #I | m1024 |

DO NOT support LOCK

This instruction is only available when the processor is executing in enclave mode. It shall not be executed outside of an enclave; doing so results in a general protection fault (#GP).

The `EREPORT` structure must be aligned to a 64-byte boundary in memory. Failure to provide a correctly aligned memory destination shall result in the instruction failing and setting the appropriate error code in the destination register. The destination memory region must be located within the enclave's linear address space and must be accessible for writing.
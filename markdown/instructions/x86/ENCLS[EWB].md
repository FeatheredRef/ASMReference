> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# ENCLS[EWB]

Encloses the execution of the subsequent code block within a software guard, ensuring that the enclosed code is not interrupted by asynchronous events and that the state is consistent. It is used to define a secure execution environment for Intel SGX.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| imm | #I |
| reg | #I |
| mN | #I |

DO NOT support LOCK

This instruction is ONLY available in 64-bit mode. It MUST be used within an Intel SGX enclave. Execution outside of an enclave environment SHALL result in a general protection fault (#GP).

The instruction MUST be paired with `ECLOSE` to properly terminate the enclosed section. Failure to do so may lead to inconsistent enclave state or security vulnerabilities. Ensure that the memory region containing the enclosed code is properly aligned and resident in the Enclave Page Cache (EPC) to avoid page faults during execution.
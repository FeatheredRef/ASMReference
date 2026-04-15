> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# ENCLS[EPA]

Encloses the processor in a secure enclave by transitioning execution to the Enclave Page Cache (EPC) address specified by the operand.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg | #I |

DO NOT support LOCK

This instruction is only available when the processor is operating in CPL3 and the SGX (Software Guard Extensions) feature is enabled. It cannot be executed in compatibility mode.

The EPA (Enclave Page Address) provided in the register MUST be a valid, linear address of an enclave's entry point. If the address is not valid or if the enclave is not in the correct state for entry, the instruction SHALL fail and set the carry flag (CF=1), leaving the processor in the non-enclave state. Ensure that the target enclave has been properly initialized via `EADD` and `EEXTEND` to avoid a general protection exception.
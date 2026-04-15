> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# GETSEC[SEXIT]

GETSEC[SEXIT] is used to exit the Secure Enclave (SE) and return execution to the non-secure state. It restores the processor state from the SEC-save area and transitions the execution environment back to the non-secure domain.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| #I | #I |

DO NOT support LOCK

The instruction is ONLY available when the processor is executing within a Secure Enclave. Attempting to execute GETSEC[SEXIT] outside of the secure environment SHALL result in a general-protection exception (#GP).

Execution of GETSEC[SEXIT] MUST be performed with the appropriate privilege levels associated with the enclave's security domain; otherwise, the operation is invalid. Failure to maintain the integrity of the SEC-save area prior to execution MAY lead to unpredictable processor state restoration.
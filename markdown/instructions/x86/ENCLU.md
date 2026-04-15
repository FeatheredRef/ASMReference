> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# ENCLU

Unlocks an enclave and resumes execution outside the enclave.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| #I | #I |

DO NOT support LOCK

This instruction is only available when the processor is executing within an enclave. It is used to exit the enclave and return to the untrusted application. Execution of ENCLU outside of an enclave environment will result in an #UD (Invalid Opcode) exception.

The instruction must be executed within the enclave's memory range. If the processor is not in enclave mode, the instruction is invalid. Failure to properly manage the enclave state before calling ENCLU may lead to undefined behavior in the untrusted application upon return.
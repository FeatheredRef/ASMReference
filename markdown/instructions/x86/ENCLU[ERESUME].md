> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# ENCLU[ERESUME]

Resumes the operation of a secure enclave after an asynchronous exit (AEX) occurred. It restores the processor state and returns execution to the enclave.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| imm | #I |
| reg | #I |
| memory | #I |

DO NOT support LOCK

This instruction SHALL only be executed in CPL 0. It is only available when the processor is operating in 64-bit mode.

The instruction SHALL only be used in conjunction with the SGX (Software Guard Extensions) feature set. If the processor does not support SGX or if the feature is disabled in the BIOS/UEFI, executing this instruction WILL result in an undefined operation exception (#UD).

To avoid system instability, the programmer MUST ensure that the state being resumed corresponds to a valid enclave context. Attempting to resume an enclave that has been destroyed or is not in an AEX-suspended state MAY result in a general protection fault (#GP).
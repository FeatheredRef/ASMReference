> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# ENCLU[EACCEPTCOPY]

Attempts to enter a Software Guard Extensions (SGX) enclave. If the enclave is successfully entered, the processor switches to enclave mode and begins execution at the entry point specified in the enclave's metadata. If the entry fails, the instruction returns an error code in the destination register.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| #I | r64 |

DO NOT support LOCK

This instruction is only available in 64-bit mode. It requires the SGX feature to be enabled in the processor and configured in the BIOS/UEFI.

The destination register is updated with a value indicating success (0) or a specific error code (e.g., invalid enclave, memory mapping issues) if the entry fails. The programmer SHALL check this value before proceeding to ensure the execution environment is secure. Failure to verify the return value MAY lead to executing code in a non-enclave state while assuming enclave protections are active.
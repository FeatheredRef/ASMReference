> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# ENCLS[ELDBC]

Enters the Enclave Page Cache Map (EPCM) based control structure (ELDBC) state. This instruction is used to transition a software execution context into a specific enclave-protected state for the purpose of managing enclave control structures.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| #I | #I |

DO NOT support LOCK

This instruction is only available when the processor is operating in Long Mode. It requires the processor to be in a state where Intel SGX is enabled and supported by the hardware. If the processor is in compatibility mode, the instruction is not supported.

The instruction SHALL only be executed from within a valid enclave context. Executing this instruction outside of an enclave or in an unsupported processor state SHALL result in a General Protection fault (#GP). Users MUST ensure that the enclave is properly initialized and that the EPCM entries are correctly configured to avoid unexpected termination of the execution flow.
> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# ENCLS

Enters a secure enclave by loading the enclave's state and switching the processor to enclave mode.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| m8 | #I |
| reg | #I |
| imm | #I |
| m128 | #I |

DO NOT support LOCK

The instruction SHALL only be executed in non-enclave mode. It is NOT supported in compatibility mode. Execution of ENCLS while already inside an enclave SHALL result in an #I (Invalid operation) or a General Protection fault (#GP).

To avoid execution failure, the operand SHALL be a memory operand pointing to the SGX Enclave Page Cache (EPC). The memory region MUST contain a valid enclave state structure. If the target enclave is not initialized or the processor is not in a state that allows entry, the instruction SHALL fail and return an error code in the destination register as specified by the SGX architectural state.
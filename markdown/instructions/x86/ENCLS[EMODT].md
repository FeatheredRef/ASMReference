> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# ENCLS[EMODT]

Enters a software enclave. This instruction initiates the transition from non-enclave mode to enclave mode by saving the current state and jumping to the specified entry point within the enclave.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| imm | #I |
| reg | #I |
| mN | #I |

DO NOT support LOCK

This instruction is only available when the processor is operating in Long Mode. It SHALL NOT be used in compatibility mode. The execution of `ENCLS` requires that the processor is not already executing within an enclave.

The `ENCLS` instruction SHALL fail and return an error code in `rax` if the enclave is not properly initialized or if the target entry point is invalid. To avoid unexpected execution flow, the programmer MUST verify the return value in `rax` immediately after the instruction to ensure the transition to the enclave succeeded.
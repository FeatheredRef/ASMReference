> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# ENCLU[EACCEPT]

The `ENCLU[EACCEPT]` instruction attempts to accept a secure enclave. If the enclave is in the AWAITING ACCEPT state and the calling processor is the one that entered the enclave via `EENTER`, the enclave state is transitioned to ACCEPTED. If the conditions are not met, the instruction fails and returns an error code.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| #I | reg |

DO NOT support LOCK

This instruction is only available in 64-bit mode. It requires the processor to be executing within a secure enclave environment (SGX).

The destination register must be a general-purpose register (r64) to receive the return value. A return value of 0 indicates success; any other value indicates a failure. To avoid incorrect state transitions, the developer MUST verify that the enclave was previously initialized and is currently in the AWAITING ACCEPT state. Failure to do so will result in the instruction returning an error code instead of transitioning the enclave state.
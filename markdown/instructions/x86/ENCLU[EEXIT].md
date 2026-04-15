> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# ENCLU[EEXIT]

Exits an enclave by transitioning execution to the enclave's exit handler or returning to the calling application.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| #I | #I |

DO NOT support LOCK

This instruction is only available when the processor is operating within an enclave. Execution of `EEXIT` outside of an enclave environment results in a general-protection exception (#GP).

The `EEXIT` instruction performs a synchronous exit from the enclave, meaning it clears the enclave's internal state and restores the processor to the state of the untrusted application. It SHALL be used specifically for planned exits. If an asynchronous event (such as an interrupt or exception) occurs, the processor performs an Asynchronous Enclave Exit (AEX) instead of executing `EEXIT`. Failure to ensure the enclave state is properly saved before calling `EEXIT` may lead to data loss within the enclave's private memory region.
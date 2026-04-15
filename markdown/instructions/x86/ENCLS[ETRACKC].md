> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# ENCLS[ETRACKC]

The `ENCLS[ETRACKC]` instruction is used within Intel Software Guard Extensions (SGX) to track the current state of a cached enclave page. It specifically informs the processor that a specific page should be tracked for the purpose of the enclave's execution context, allowing the hardware to monitor the page for modifications or access patterns.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| imm | #I |

DO NOT support LOCK

The `ENCLS` instruction is only available when the processor is executing in CPL3 and the current execution environment is within an SGX enclave. It SHALL NOT be executed outside of an enclave; doing so results in a general protection fault (#GP).

The `ETRACKC` leaf is specific to the enclave's internal management. If the enclave is not in a valid state to perform tracking operations, or if the memory range targeted is outside the Enclave Page Cache (EPC), the instruction SHALL return an error code in `rC` (the destination register for `ENCLS` results). Failure to verify the return value of `ENCLS` may lead to silent failures in page tracking, resulting in inconsistent enclave state or unexpected memory faults.
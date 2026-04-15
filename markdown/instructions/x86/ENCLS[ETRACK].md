> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# ENCLS[ETRACK]

The `ENCLS[ETRACK]` instruction is used within the Intel SGX enclave to track the execution of a specific code path by updating the internal state of the enclave's tracking mechanism. It verifies that the current execution context is within a secure enclave and updates the tracking structures accordingly.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| imm | #I |
| reg | #I |
| mN | #I |

DO NOT support LOCK

This instruction SHALL only be executed from within an SGX enclave. If executed outside of an enclave, the instruction is NOOP. It is only supported in 64-bit mode.

The instruction depends on the correct initialization of the enclave's internal tracking state; failure to properly initialize the environment MAY result in the instruction failing to track the execution path correctly without triggering a general protection fault.
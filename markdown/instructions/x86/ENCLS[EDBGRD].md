> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# ENCLS[EDBGRD]

Enters a clandestine loop by saving the current state and transitioning execution to a secure enclave. It verifies that the processor is in a state compatible with enclave entry and updates the internal state to reflect the transition.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| #I | #I |

DO NOT support LOCK

The instruction is ONLY available when the processor is operating in 64-bit mode. It SHALL NOT be used in compatibility mode. The instruction REQUIRES the processor to be in CPL 3 and the `SgxEnable` bit in the `IA32_FEATURE_CONTROL` MSR to be set.

If the processor is not in the required state (e.g., not in CPL 3 or SGX is disabled), the instruction SHALL trigger a general protection exception (#GP). The destination address for the enclave entry must be valid and aligned according to the enclave's architectural specifications to avoid memory faults.
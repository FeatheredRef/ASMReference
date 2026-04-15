> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# ENCLS[EEXTEND]

Enters a secure enclave by extending the current enclave's state. It transitions execution from non-enclave mode to enclave mode or from one enclave to another, provided the target enclave is initialized and the transition is permitted by the enclave's attributes and the processor's security state.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| imm | #I |
| reg | #I |
| mN | #I |

DO NOT support LOCK

This instruction is ONLY available in 64-bit mode. It SHALL NOT be executed in compatibility mode. The instruction requires the processor to support Intel SGX (Software Guard Extensions). Execution outside of a valid SGX environment or with invalid transition parameters WILL trigger a #GP (General Protection Fault).

The instruction requires the target enclave's identity and attributes to be correctly configured in the Enclave Page Cache Map (EPCM). Failure to provide a valid target address or attempting to enter an enclave that has been destroyed WILL result in a failure of the transition. The state of the processor is saved and restored during the transition; users MUST ensure the stack is correctly aligned to avoid stability issues during the switch to enclave mode.
> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# ENCLU[EMODPE]

Enter Conditional LU (Enclave Update) with EMODPE. This instruction resumes execution within an enclave after a successful `EEXIT` or an asynchronous exit (AEX), provided that the conditions specified by the EMODPE (Enter Modified Processor State) parameter are met. It restores the processor state from the State Save Area (SSA) inside the enclave.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| imm | #I |
| reg | #I |
| mN | #I |

DO NOT support LOCK

This instruction is ONLY available when the processor is operating in SGX-enabled mode and is specifically used to transition the processor back into enclave mode. It SHALL only be executed from the enclave's trusted code or by the SGX hardware mechanisms during state restoration.

The `EMODPE` operand is a bitmask that specifies which parts of the processor state (such as registers or flags) are to be modified or restored during the transition. Failure to correctly specify the EMODPE bits relative to the current state of the State Save Area (SSA) MAY result in an unexpected processor state or a general protection fault (#GP). This instruction is not supported in compatibility mode.
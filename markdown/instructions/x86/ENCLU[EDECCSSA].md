> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# ENCLU[EDECCSSA]

Loads the value from the source operand into the specified destination operand and then resumes execution from the enclave that was previously suspended by an `EENTER` or `ERESUME` instruction.

The following table covers the supported source and destinations.

| source | destination(s) |
| :--- | :--- |
| reg | rN |
| imm | rN |
| mN | rN |

DO NOT support LOCK

This instruction is only available when the processor is operating in 64-bit mode and is specifically part of the Intel SGX (Software Guard Extensions) instruction set. It SHALL only be executed outside of an enclave (non-enclave mode). Execution of this instruction while inside an enclave SHALL result in a general protection exception (#GP(0)).

The `ENCLU` instruction is used specifically to return control to an enclave after a synchronization or exception handling event. If the processor is not in a state where it can resume an enclave (e.g., no enclave was previously suspended), the instruction SHALL fail to resume the enclave, but the initial load of the source operand into the destination register will still occur. Ensure that the state of the processor is consistent with an active SGX session to avoid unexpected behavior during the transition back to enclave mode.
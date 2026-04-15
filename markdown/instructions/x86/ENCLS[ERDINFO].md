> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# ENCLS[ERDINFO]

The `ENCLS[ERDINFO]` instruction retrieves information about the Enclave Page Cache Map (EPCM) for the current enclave. It reads the EPCM entry associated with the linear address specified in the operand and stores the result in a destination memory buffer.

The following table describes the supported sources and destinations.

| source | destination(s) |
| :--- | :--- |
| reg | m8 |

DO NOT support LOCK

This instruction is only available when the processor is executing within an SGX enclave. If executed outside of an enclave, the instruction is treated as a NOP. The instruction requires the processor to be in 64-bit mode.

The destination memory buffer MUST be a valid linear address within the enclave. If the provided address is invalid or points to memory that cannot be written to, the behavior is undefined and may trigger a general protection fault (#GP). The user SHALL ensure that the memory buffer is properly aligned to avoid performance penalties or faults on specific hardware implementations.
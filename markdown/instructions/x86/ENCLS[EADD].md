> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# ENCLS[EADD]

Adds a 64-bit memory address to the enclave page cache map (EPCM) tracking for the current enclave, specifically updating the enclave's internal page tracking structures to include the specified address.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| imm | #I |
| reg | #I |
| m8 | #I |
| m32 | #I |
| m64 | #I |
| qword | #I |

DO NOT support LOCK

This instruction is ONLY available when the processor is executing within an enclave (Enclave Mode). Execution outside of an enclave SHALL result in a general protection fault (#GP). It is not supported in compatibility mode.

The `EADD` leaf is used exclusively with the `ENCLS` (Execute Enclave Command) instruction. The specific operation is triggered by loading the value `EADD` into the `EAX` register before executing `ENCLS`. Failure to provide the correct leaf value in `EAX` SHALL cause the instruction to return an error code in `EAX` rather than performing the memory addition.

The address provided MUST be 4KB page-aligned. If the address is not aligned to a 4KB boundary, the instruction SHALL fail and return an error code in `EAX`.
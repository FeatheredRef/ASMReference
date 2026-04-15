> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# ENCLS[EINIT]

Initializes a convex enclave. This instruction transitions the processor into the enclave mode and establishes the enclave's memory layout and security properties based on the Enclave Page Cache Map (EPCM).

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| imm | #I |
| reg | #I |
| mN | #I |

DO NOT support LOCK

This instruction SHALL only be executed in non-enclave mode. It is only supported in 64-bit mode. Execution of `ENCLS[EINIT]` while already inside an enclave SHALL result in an invalid opcode exception.

The `EINIT` leaf MUST be called after the enclave has been fully loaded into the EPC using `EADD` and `EEXTEND` instructions. If the enclave's identity or measurements do not match the expected values stored in the SIGSTRUCT, the instruction SHALL fail and return an error code in `rax`. Failure to properly align the enclave's memory pages according to the EPC requirements WILL result in an initialization error.
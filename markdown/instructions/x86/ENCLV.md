> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# ENCLV

Enter Callout Volume. This instruction is used to enter a callout volume, allowing the processor to execute a specific set of functions within a secure enclave. It validates the target callout volume and transfers execution to the specified entry point.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| imm | #I |
| reg | #I |
| mN | #I |

DO NOT support LOCK

This instruction is ONLY available when the processor is operating in 64-bit mode. It is NOT supported in compatibility mode.

The instruction requires that the processor is currently executing within a secure enclave context. If the processor is not in enclave mode, the instruction SHALL trigger a general protection fault (#GP).

To avoid execution failures, ensure that the target callout volume is correctly configured in the enclave's metadata and that the entry point is valid and aligned. Failure to provide a valid target volume SHALL result in an error code returned in the destination register.
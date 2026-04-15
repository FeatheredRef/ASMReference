> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# ENCLV[ESETCONTEXT]

Sets the context for a secure enclave by loading the specified state into the enclave's internal context. This instruction is used within the Software Guard Extensions (SGX) framework to initialize or restore the execution environment of an enclave.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| m8 | internal state |

DO NOT support LOCK

This instruction SHALL only be executed in CPL 3. It is only available when the processor is in 64-bit mode and the SGX feature is enabled in the `CPUID` and `IA32_FEATURE_CONTROL` MSR. Attempting to execute this instruction outside of an active enclave or at a higher privilege level SHALL result in a general protection fault (#GP).

The memory operand MUST be a valid address within the enclave's linear address space; accessing memory outside the enclave boundary SHALL trigger a page fault or a general protection fault. Ensure that the enclave is properly initialized via `EENTER` before attempting to set the context to avoid unpredictable behavior.
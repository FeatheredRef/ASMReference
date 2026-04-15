> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# ENCLV[EINCVIRTCHILD]

Increments the virtual child count associated with the current enclave, used in conjunction with Intel® Software Guard Extensions (SGX) to manage the number of active virtual children for resource tracking and security boundaries.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| #I | #I |

DO NOT support LOCK

This instruction is only available when the processor is operating in 64-bit mode and the SGX feature is enabled in the processor. It MUST be executed within an enclave; execution outside of an enclave environment SHALL result in a general protection (#GP) fault.

The instruction does not take explicit operands as the target is implicitly the internal enclave state managed by the hardware. Failure to properly manage virtual child counts may lead to enclave termination if the architectural limits of the SGX version are exceeded.
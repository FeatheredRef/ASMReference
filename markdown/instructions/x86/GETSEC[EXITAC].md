> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# GETSEC[EXITAC]

GETSEC[EXITAC] is used to exit the Enclave Dynamic Memory Management (EDMM) state. It clears the internal state associated with the current enclave's dynamic memory management and returns the processor to a state where further dynamic memory operations are disallowed until another GETSEC operation is performed.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| #I | #I |

DO NOT support LOCK

This instruction is only available when the processor is executing in 64-bit mode. It SHALL only be executed within an enclave; if executed outside of an enclave, it will result in a general protection fault (#GP).

To avoid unexpected #GP exceptions, the software MUST verify that the processor supports the SGX (Software Guard Extensions) feature and that the current execution context is within a valid enclave prior to invoking this instruction.
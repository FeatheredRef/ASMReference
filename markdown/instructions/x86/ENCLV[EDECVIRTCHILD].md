> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# ENCLV[EDECVIRTCHILD]

This instruction is used within the Intel Software Guard Extensions (SGX) to remove a virtual child from the enclave's child tracking mechanism. It updates the internal state of the enclave to decrement the count of active virtual children associated with the current enclave execution context.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| imm | #I |
| reg | #I |
| memory | #I |

DO NOT support LOCK

This instruction is ONLY available when the processor is executing in enclave mode. It SHALL NOT be executed outside of an enclave, as doing so will result in a general protection fault (#GP).

To avoid execution failures, the programmer MUST ensure that the instruction is called only after a virtual child has been successfully created. Attempting to decrement the virtual child count below zero is undefined and MAY lead to unexpected enclave state corruption or termination.
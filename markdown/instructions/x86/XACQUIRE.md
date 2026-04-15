> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# XACQUIRE

The XACQUIRE instruction acts as a memory acquire fence. It ensures that all memory reads and writes following the XACQUIRE instruction in program order are not executed until the XACQUIRE instruction itself has completed.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| #I | #I |

DO NOT support LOCK

This instruction is only available in 64-bit mode. It is NOT supported in compatibility mode.

XACQUIRE is used in conjunction with XRELEASE to implement a release-acquire semantics for synchronization primitives. Failure to pair these instructions correctly in a multi-processor environment MAY lead to memory ordering violations. Because this instruction does not take operands, it cannot trigger general protection faults (#GP) related to memory access or invalid register usage.
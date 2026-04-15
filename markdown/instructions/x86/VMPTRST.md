> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VMPTRST

Stores the 64-bit value of the current VMCS pointer into a specified memory location.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| Internal State | m8 |

DO NOT support LOCK

This instruction SHALL only be executed in VMX root operation. It is NOT supported in VMX non-root operation.

The destination memory operand MUST be a qword aligned to a 64-bit boundary to avoid potential performance penalties or alignment checks. Failure to execute this instruction in VMX root operation SHALL result in an #UD (Undefined Opcode) exception.
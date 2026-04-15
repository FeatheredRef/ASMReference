> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VMXON

Enables Virtual Machine Extensions (VMX) operation by loading the VMXON region physical address from the operand into the VMX-ON region pointer.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| m64 | Internal State |

DO NOT support LOCK

VMXON SHALL only be executed in VMX basic operation. It is NOT supported in compatibility mode. The instruction SHALL be executed with CPL=0. If the processor is already in VMX operation, attempting to execute VMXON SHALL cause a #GP(0) exception.

The operand MUST be a 64-bit physical address of the VMXON region. This region MUST be 4KB-aligned. The memory region MUST be present and read/write accessible. Failure to meet alignment or accessibility requirements SHALL result in a #GP(0) exception. The VMXON region MUST be initialized according to the architectural specifications before execution to avoid undefined behavior.
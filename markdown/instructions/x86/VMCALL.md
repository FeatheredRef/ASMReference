> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VMCALL

Calls the VMX root operation or triggers a VM exit to the Virtual Machine Monitor (VMM).

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| #I | #I |

DO NOT support LOCK

VMCALL is available only when the processor is operating in VMX non-root operation. If executed in VMX root operation, the instruction is ignored (it acts as a NOP).

To avoid unexpected behavior, the software SHALL ensure that the processor is in VMX non-root operation before invoking VMCALL to successfully transfer control to the VMM. Because VMCALL does not take operands, any attempt to specify a source or destination will result in an invalid operation.
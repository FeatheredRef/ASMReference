> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VMRESUME

Resumes guest operation by loading the guest state from the Virtual Machine Control Structure (VMCS) specified by the current VPID and the active VMCS pointer.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| #I | #I |

DO NOT support LOCK

This instruction is ONLY available when the processor is operating in VMX operation. It SHALL be executed only while in VMX root operation. Executing VMRESUME outside of VMX root operation results in an invalid opcode exception (#UD).

To avoid VM-entry failures, the software MUST ensure that the VMCS is in a state consistent with the processor's architectural requirements. If the guest state loaded from the VMCS is inconsistent (e.g., invalid segment registers or control register values), a VM-entry failure occurs, and the processor remains in VMX root operation. The cause of the failure is recorded in the VM-entry failure reason field of the VMCS.
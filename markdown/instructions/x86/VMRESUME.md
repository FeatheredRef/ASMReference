> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VMRESUME

Resumes guest operation by loading the guest state from the VMCS specified by the current VMCS pointer and transferring control to the guest.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| #I | #I |

DO NOT support LOCK

This instruction is ONLY available in VMX root operation. Executing `VMRESUME` outside of VMX root operation SHALL cause an invalid opcode exception.

To avoid an invalid transition or a VM-entry failure, the user MUST ensure that the current VMCS is in a state that allows VM-entry. If the VMCS is not in the "VM-entry" state (e.g., if a previous `VMLAUNCH` or `VMRESUME` failed), the instruction SHALL fail. The user SHOULD verify that the guest state fields in the VMCS are consistent with the architectural requirements of the target guest mode to prevent a VM-entry failure.
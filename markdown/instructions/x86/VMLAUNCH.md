> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VMLAUNCH

The `VMLAUNCH` instruction initiates VMX operation by performing a VM entry. It transitions the processor from VMX root operation to VMX non-root operation by loading the guest state from the Virtual Machine Control Structure (VMCS) pointed to by the current VMCS pointer.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| #I | #I |

DO NOT support LOCK

The `VMLAUNCH` instruction SHALL only be executed when the processor is in VMX root operation. It is only available in 64-bit mode or compatibility mode. Execution of this instruction while the processor is not in VMX root operation results in an undefined operation (typically a general protection fault).

To avoid VM entry failures, the VMCS MUST be properly initialized and the `VMLAUNCH` instruction SHALL only be used for the initial entry into a guest. Subsequent entries MUST use the `VMRESUME` instruction. The processor performs a series of consistency checks on the VMCS fields; if any check fails, the instruction DOES NOT transition to non-root operation and sets the return value in `r64` to the error code. Failure to ensure that the guest state is consistent with the host state and the processor's current capabilities will result in a failed VM entry.
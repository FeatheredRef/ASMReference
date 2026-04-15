> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VMPTRLD

Loads the VM-pointer index from a specified memory location into the virtual machine control structure (VMCS) pointer.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| m64 | internal state |

DO NOT support LOCK

The instruction is ONLY available when the processor is operating in VMX operation. It SHALL be executed while the processor is in root operation. Use of this instruction outside of VMX root operation will result in an invalid opcode exception.

To avoid VM-exit or general protection faults, the memory operand MUST be a valid 64-bit address containing a valid VMCS pointer. The pointer MUST be aligned on a 64-bit boundary. Failure to ensure that the memory location contains a valid VMCS pointer (as verified by VMPTRST) MAY lead to undefined behavior during subsequent VM entry attempts.
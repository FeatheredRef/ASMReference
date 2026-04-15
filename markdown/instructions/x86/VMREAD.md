> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VMREAD

VMREAD reads a specified field from the Virtual Machine Control Structure (VMCS) pointed to by the current processor's VMCS pointer and stores the value in the destination register.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| imm | r64 |

DO NOT support LOCK

The instruction SHALL only be executed in VMX root operation. Execution in VMX non-root operation or in a mode where VMX is not enabled SHALL result in an invalid opcode exception (#UD).

The `imm` operand MUST be a valid VMCS field encoding. If the `imm` value does not correspond to a valid field, the instruction SHALL trigger an invalid opcode exception (#UD) or set the appropriate error flags depending on the processor implementation.

When using VMREAD, the programmer MUST ensure that a VMCS is currently active (i.e., `VMPTRLD` was previously executed successfully). If no VMCS is current, the instruction SHALL fail.
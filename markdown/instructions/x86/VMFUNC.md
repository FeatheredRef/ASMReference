> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VMFUNC

VMFUNC allows a guest virtual machine to invoke a function provided by the virtual machine monitor (VMM) without causing a VM exit. The behavior depends on the function index specified in the EAX register.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg (eax) | #I |

DO NOT support LOCK

This instruction SHALL only be executed in VMX non-root operation. If executed in VMX root operation, it is treated as a NOP. The instruction is only supported when the "Enable VMFUNC" VM-execution control is set to 1 in the VMCS.

The function index in EAX MUST be supported by the VMM. If the function index is not supported or is invalid, the instruction triggers a general protection fault (#GP). For EPTP switching (Function 0), the VMFUNC instruction requires that the "EPTP switching" VM-execution control be enabled and a valid EPTP list be configured in the VMCS. Failure to configure the EPTP list prior to execution will result in an undefined behavior or a #GP.
> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# FRSTOR

Restores the state of the x87 floating-point unit (FPU) by loading the FPU register stack and the control word from a specified memory location.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| m108 | FPU Register Stack and Control Word |

DO NOT support LOCK

This instruction is available in 64-bit mode, but it operates exclusively on the x87 FPU state. In x86-64, the FPU is maintained for compatibility, but the architectural state is handled as a legacy entity.

It is MANDATORY to ensure that the memory region m108 is correctly aligned and contains a valid FPU state save-area created by FSAVE. Failure to provide a valid save-area will result in an inconsistent FPU state. Use of this instruction in environments where the FPU state is managed by XSAVE/XRSTOR may lead to state corruption if the x87 component is not correctly tracked.
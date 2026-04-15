> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# XRESLDTRK

Resumes the execution of a guest by reloading the Task Register (TR) and the Guest State Area from the specified memory location.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| m8 | reg |
| #I | #I |

DO NOT support LOCK

This instruction is only available in VMX root operation. It SHALL be used exclusively to restore the task register state during a VM entry or guest state restoration process.

The instruction REQUIRES that the memory operand is aligned to the boundaries defined by the architectural state of the Task Register. Failure to provide a valid memory reference SHALL result in a general protection fault (#GP) or a page fault (#PF) depending on the memory accessibility. If the memory operand is not properly aligned, it MAY result in alignment check exceptions if alignment checking is enabled.
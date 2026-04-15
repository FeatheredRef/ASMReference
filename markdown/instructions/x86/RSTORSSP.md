> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# RSTORSSP

RSTORSSP restores the stack pointer from a shadow stack pointer. It pops the value from the shadow stack and loads it into the RSP register.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| m64 | r64 |

DO NOT support LOCK

This instruction is only available when Control-flow Enforcement Technology (CET) is enabled. It operates exclusively in 64-bit mode.

The instruction MUST be used in conjunction with the shadow stack mechanism; if the shadow stack is not enabled or if the operation violates shadow stack integrity, a Control Protection Exception (#CP) SHALL occur. The value being restored MUST be a valid shadow stack pointer.
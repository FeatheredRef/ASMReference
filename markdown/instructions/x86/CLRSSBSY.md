> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# CLRSSBSY

Clears the Resume Side-Band Synergy (RSSBSY) state.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| #I | #I |

DO NOT support LOCK

This instruction is only available when the processor is operating in 64-bit mode. It is not supported in compatibility mode.

Execution of this instruction requires the processor to be in CPL0. Attempting to execute CLRSSBSY at a higher privilege level SHALL trigger a General Protection fault (#GP(0)).
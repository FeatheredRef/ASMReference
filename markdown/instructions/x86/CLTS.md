> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# CLTS

Clears the TS (Task Switched) flag in CR0.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| #I | CR0 |

DO NOT support LOCK

This instruction is available only in 64-bit mode and compatibility mode. It is a privileged instruction and SHALL only be executed at CPL=0; otherwise, it triggers a general protection exception (#GP).

The CLTS instruction is specifically used to clear the TS bit to allow the processor to execute instructions that require a valid state, such as those involving the FPU or SSE, without triggering a device-not-available (#NM) exception. Failure to clear this bit after a task switch in certain environments MAY result in unexpected #NM exceptions when accessing floating-point units.
> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# CLUI

Clears the User-Level Interrupts (ULI) state. This instruction resets the internal state associated with user-level interrupt handling, effectively disabling the ability for the processor to receive user-level interrupts.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| #I | #I |

DO NOT support LOCK

This instruction is only available when the processor is operating in 64-bit mode. It is NOT supported in compatibility mode.

Failure to execute CLUI before transitioning to a lower privilege level or changing interrupt configurations may result in unexpected delivery of pending user-level interrupts. Ensure that the processor state is synchronized with the intended interrupt masking policy to avoid race conditions in interrupt handling.
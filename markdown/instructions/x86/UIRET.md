> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# UIRET

Returns from an interrupt in the context of the User-Mode Interrupt (UIRET) mechanism, restoring the instruction pointer and flags from the stack and transitioning the processor back to the original execution state.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| m64 | reg |

DO NOT support LOCK

The instruction is only available when the processor is operating in 64-bit mode and the User-Mode Interrupt feature is enabled. It SHALL NOT be used in compatibility mode.

The stack pointer (RSP) MUST be properly aligned to the requirements of the UIRET return sequence to avoid a General Protection exception (#GP). Since this instruction pops values from the stack to restore the execution state, the stack MUST contain valid return addresses and flags; failure to maintain stack integrity WILL result in the processor loading an invalid instruction pointer, leading to a #GP or a crash.
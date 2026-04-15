> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# HLT

Stops instruction execution by placing the processor in a halt state. The processor remains in this state until a wake-up event occurs, such as an external interrupt, a Non-Maskable Interrupt (NMI), or a RESET signal.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| #I | #I |

DO NOT support LOCK

HLT is available in both 64-bit mode and compatibility mode. In protected mode or 64-bit mode, the instruction SHALL generate a General Protection (#GP) exception if executed with CPL > 0 (i.e., it is a privileged instruction).

To avoid an unexpected system hang, the IF (Interrupt Flag) in RFLAGS SHALL be set to 1 before executing HLT. If interrupts are disabled (IF=0) and no NMI is pending, the processor will enter a state from which it cannot be woken by standard external interrupts, requiring a hardware reset to resume execution.
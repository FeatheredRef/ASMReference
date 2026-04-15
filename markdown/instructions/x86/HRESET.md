> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# HRESET

Resets the hardware state of the processor to its initial power-on configuration, effectively triggering a hard reset of the execution environment.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| #I | #I |

DO NOT support LOCK

The instruction is ONLY available when the processor is operating in compatibility mode. Execution in 64-bit mode SHALL result in an invalid opcode exception.

The instruction triggers a complete pipeline flush and clears all volatile register states, including control registers and debug registers. This operation is non-maskable; interrupts SHALL be ignored during the reset sequence. Ensure that all critical data is committed to non-volatile memory or persistent storage before execution, as any data residing exclusively in registers or cache that is not flushed SHALL be lost.
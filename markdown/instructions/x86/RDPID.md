> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# RDPID

Reads the processor ID (PID) of the current logical processor into the destination operand.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| #I | r64 |

DO NOT support LOCK

This instruction is available only in 64-bit mode. It is NOT supported in compatibility mode or 32-bit protected mode.

The RDPID instruction is only available on processors that implement the PID feature. Attempting to execute this instruction on a processor that does not support it SHALL result in an invalid opcode exception (#UD). To avoid this, software MUST check the CPUID leaf for the presence of the RDPID bit before execution.
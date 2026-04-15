> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# GETSEC[WAKEUP]

The `GETSEC[WAKEUP]` instruction wakes up the processor from a low-power state or triggers a specific security-related wake event, depending on the processor implementation.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| #I | #I |

DO NOT support LOCK

This instruction is only available when the processor is operating in 64-bit mode. Execution in compatibility mode or 32-bit mode SHALL result in an invalid opcode exception.

The instruction is designed for use in specific security contexts; executing this instruction without the appropriate hardware support or in an incorrect power state MAY result in an ignored operation or a general protection fault (#GP) depending on the CPU stepping. Ensure that the processor supports the `GETSEC` feature flag in `CPUID` before execution to avoid undefined behavior.
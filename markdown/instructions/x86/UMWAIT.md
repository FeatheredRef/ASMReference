> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# UMWAIT

The instruction suspends the execution of the processor until a specified time period has elapsed, a specified event occurs, or an interrupt is received. It allows the processor to enter a low-power state while waiting for a condition to be met.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| imm | #I |
| reg | #I |

DO NOT support LOCK

The `UMWAIT` instruction SHALL only be executed when the `WAITPKG` feature is supported by the processor and enabled. It is available in 64-bit mode. Execution of `UMWAIT` in compatibility mode or 32-bit mode may result in an invalid opcode exception if the feature is not explicitly supported for those modes.

To avoid unexpected behavior or immediate wake-up, the user SHALL ensure that the `CPL` (Current Privilege Level) is 3; executing `UMWAIT` at a higher privilege level (CPL < 3) may result in the instruction being treated as a `NOP` or triggering a general protection fault depending on the processor implementation. Additionally, the user SHOULD be aware that the time delay is not precise and is subject to the processor's power management state and clock frequency.
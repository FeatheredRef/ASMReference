> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# MWAIT

Monitors the processor to wait for a write access to a monitored address range. The processor enters a sleep state and remains in that state until a write occurs to the monitored address or an interrupt occurs.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg | #I |

DO NOT support LOCK

MWAIT is a privileged instruction. If executed in CPL > 0, it SHALL trigger a General Protection exception (#GP(0)). Its availability depends on the processor implementation and may be disabled via the `MWAIT` bit in the `CR4` register or via BIOS settings. It MUST be used in conjunction with `MONITOR` to define the address range being tracked.

To avoid unintended behavior or system hangs, the programmer MUST ensure that `MONITOR` is called immediately before `MWAIT`. If a write access to the monitored address occurs between the execution of `MONITOR` and `MWAIT`, the processor MAY not enter the sleep state. The instruction is not supported in all x86-64 implementations; therefore, the software SHOULD check for support via `CPUID` to avoid invalid opcode exceptions.
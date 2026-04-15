> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# UMONITOR

Sets the address of a memory region to be monitored for writes. The processor monitors the specified memory address; if a write access occurs to the monitored address, the processor clears the monitor state, which can then be detected by the `UMWAIT` or `TPAUSE` instructions.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg | #I |
| m64 | #I |
| imm | #I |

DO NOT support LOCK

The `UMONITOR` instruction is only available in 64-bit mode. It is NOT supported in compatibility mode.

To avoid incorrect behavior or failure of the monitoring mechanism, the following details MUST be observed:
- The memory address specified in the register MUST be aligned to a cache line boundary; otherwise, the behavior is undefined.
- A call to `UMONITOR` MUST be followed by a waiting instruction (such as `UMWAIT`) before any other instruction that might trigger a cache line eviction or modification occurs.
- If the processor is in a state where monitoring is already active, `UMONITOR` will overwrite the previously monitored address.
- Memory monitoring is local to the logical processor; writes from other processors will trigger the monitor clear, but the setup is specific to the core executing the instruction.
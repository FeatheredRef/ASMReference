> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# MONITOR

Sets the address range to be monitored for write accesses. When a write occurs to the monitored address, the processor clears the wait state, allowing a subsequent MWAIT instruction to terminate its sleep state.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| m8 | #I |
| m16 | #I |
| m32 | #I |
| m64 | #I |
| reg | #I |
| imm | #I |
| #I | m8 |
| #I | m16 |
| #I | m32 |
| #I | m64 |

DO NOT support LOCK

The MONITOR instruction is only available in 64-bit mode. It is NOT supported in compatibility mode. The address monitored is aligned to a cache line boundary; any address within the same cache line as the operand will trigger the wake-up event.

The address operand MUST be a valid memory address. If the address is not aligned to a cache line boundary, the processor monitors the cache line containing the address. A MWAIT instruction MUST follow a MONITOR instruction to effectively enter a low-power state; executing MWAIT without a preceding MONITOR instruction results in undefined behavior or fails to enter the intended state. The monitor state is cleared if an interrupt occurs or if a write access to the monitored range is detected.
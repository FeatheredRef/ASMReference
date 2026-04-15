> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# SYSRET

SYSRET executes a return from ring 0 to ring 3. It loads RIP from RCX and RSP from the stack, pops the remaining stack contents, and switches the privilege level to 3.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| imm | #I |
| mN | #I |

DO NOT support LOCK

SYSRET is ONLY available in 64-bit mode. It SHALL NOT be used in compatibility mode. The instruction assumes that the target is a 64-bit flat mode segment; it does not load segment selectors from the stack, but instead sets the CS and SS selectors based on predefined values derived from the current state.

To avoid a General Protection (#GP) exception, the processor MUST be in 64-bit mode and the target privilege level MUST be 3. If the target is not a valid transition to ring 3, the instruction WILL fail. Ensure that RCX contains the correct return address before execution, as SYSRET does not pop the return address from the stack like IRET.
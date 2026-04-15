> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# KANDND

Performs a bitwise AND of the source and destination operands, and then performs a bitwise NOT on the result, storing the final value in the destination.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| imm | reg |
| reg | mN |
| imm | mN |

DO NOT support LOCK

This instruction is only available in 64-bit mode. It is NOT supported in compatibility mode or 32-bit mode.

The instruction does NOT affect any EFLAGS bits. Ensure that the memory operand size matches the register size to avoid truncated results.
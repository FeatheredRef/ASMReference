> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PTEST

Performs a bitwise logical AND between two operands and sets the ZF (Zero Flag) based on the result. If the result of the AND operation is zero, ZF is set to 1; otherwise, ZF is cleared to 0.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| mN | reg |
| reg | mN |

DO NOT support LOCK

PTEST is only available in 64-bit mode. It is NOT supported in compatibility mode or 32-bit mode.

The instruction does not modify the destination operands; it only affects the ZF flag. This avoids the destructive nature of the standard TEST instruction when dealing with specific register constraints. Users SHALL ensure the processor is operating in 64-bit mode to avoid illegal instruction exceptions.
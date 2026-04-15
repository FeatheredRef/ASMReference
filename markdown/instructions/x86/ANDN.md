> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# ANDN

Performs a bitwise AND operation between the NOT of the first source operand and the second source operand, storing the result in the destination operand.

The following table covers the supported source and destination operands:

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| imm | reg |

DO NOT support LOCK

This instruction is only available when the processor is operating in 64-bit mode. It is NOT supported in compatibility mode or 32-bit mode.

The destination register MUST be the same as the second source operand to maintain the standard `ANDN` functional pattern, although the ISA allows the destination to be a different register. This instruction does not affect the EFLAGS register; specifically, the CF, OF, PF, and SF flags are NOT updated.
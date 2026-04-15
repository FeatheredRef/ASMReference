> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# KSHIFTLW

Shifts the lower 32 bits of the source operand to the left by the count specified in the second operand. The result is stored in the destination operand.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| imm | reg |

DO NOT support LOCK

This instruction is only available in 64-bit mode. It is NOT supported in compatibility mode.

The shift count is masked to 5 bits (u5), meaning only values from 0 to 31 are used regardless of the actual value provided in the register or immediate.

Since this is a 32-bit operation within a 64-bit register context, the upper 32 bits of the destination register are not modified. Failure to account for the preserved upper 32 bits may lead to logic errors when treating the destination register as a qword.
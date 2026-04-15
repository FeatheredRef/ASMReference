> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# SARX

Shifts the source operand right arithmetically by the count specified in the immediate operand, then stores the result in the destination operand.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| mN | reg |

DO NOT support LOCK

This instruction is available only in 64-bit mode. It is NOT supported in compatibility mode.

The immediate operand SHALL be a 5-bit value. If the operand size is 64 bits, the shift count is masked to 5 bits (0-31); if 32 bits, it is masked to 4 bits (0-15). This instruction does NOT affect the EFLAGS register, unlike the standard SAR instruction.
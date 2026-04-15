> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# KADDQ

Adds two 64-bit quad-word values from the source and destination operands and stores the result in the destination operand.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| imm | reg |
| mem | reg |

DO NOT support LOCK

This instruction is available only in 64-bit mode. It operates on XMM registers and does not affect the EFLAGS register.

The immediate operand MUST be a signed 8-bit integer, which is sign-extended to 64 bits before the addition. Using this instruction in 32-bit mode or compatibility mode SHALL result in an invalid opcode exception.
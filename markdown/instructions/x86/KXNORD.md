> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# KXNORD

Performs a bitwise XOR operation between a register and a memory operand, then performs a logical NOT operation on the result, storing the final value in the destination.

The following table specifies the supported sources and destinations:

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| m8/m16/m32/m64 | reg |
| reg | m8/m16/m32/m64 |

DO NOT support LOCK

This instruction SHALL be used only in 64-bit mode. It is NOT supported in compatibility mode.

To avoid unexpected behavior, ensure that the size of the register matches the size of the memory operand (mN); mismatching operand sizes SHALL result in an invalid opcode exception.
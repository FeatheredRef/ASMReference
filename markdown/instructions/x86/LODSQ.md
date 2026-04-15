> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# LODSQ

Loads a qword from the memory location pointed to by the RSI register into the RAX register, then increments or decrements the RSI register by 8 bytes based on the current value of the Direction Flag (DF).

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| m8 [RSI] | r64 (RAX) |
| #I | imm |
| #I | reg |

DO NOT support LOCK

This instruction is available in 64-bit mode and compatibility mode. It requires the RSI register to be used as the source address and the RAX register as the destination.

The behavior of the RSI register after the load depends on the DF flag: if DF=0, RSI is incremented by 8; if DF=1, RSI is decremented by 8. To ensure the correct movement direction, the programmer SHALL explicitly clear (CLD) or set (STD) the Direction Flag before execution. Since this instruction implicitly uses RSI and RAX, it SHALL NOT be used if those registers hold data that must be preserved.
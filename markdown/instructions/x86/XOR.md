> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# XOR

Performs a bitwise logical exclusive OR operation on the source and destination operands. The result is stored in the destination.

The following table covers the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| imm | reg |
| reg | mN |
| imm | mN |

Support LOCK

The instruction is supported in 64-bit mode, compatibility mode, and 32-bit mode. When operating on mN, the operand size must be specified (word, dword, or qword) to determine the width of the operation.

XORing a register with itself (e.g., `XOR r64, r64`) is a common idiom to zero the register. In x86-64, this is recognized by the hardware as a zeroing idiom, which does not modify the upper bits of the register in some implementations and avoids a data dependency on the previous value of the register.

The ZF (Zero Flag) is set if the result of the operation is zero; otherwise, it is cleared. The SF (Sign Flag) is set equal to the most significant bit of the result. The OF (Overflow Flag) and CF (Carry Flag) are always cleared.
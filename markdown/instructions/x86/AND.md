> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# AND

Performs a bitwise logical AND operation on the first operand and the second operand, storing the result in the destination operand.

The following table covers the supported source and destination operands:

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| imm | reg |
| reg | mN |
| imm | mN |
| reg | reg |

Support LOCK

The instruction is available in 16-bit, 32-bit, and 64-bit operand sizes. In 64-bit mode, the operand size is determined by the operand size override prefix or the default qword size.

The ZF (Zero Flag) is set if the result is zero; otherwise, it is cleared. The SF (Sign Flag) is set if the most significant bit of the result is 1; otherwise, it is cleared. The JF (Carry Flag) and OF (Overflow Flag) are cleared.

When using an immediate operand, the immediate must be sign-extended to the operand size. To avoid unintended bit masking, ensure the immediate value matches the intended bit-width of the target register or memory location. Use of the `LOCK` prefix is only applicable when the destination is a memory operand.
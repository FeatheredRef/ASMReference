> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# KORTESTW

Performs a logical AND between the source and destination operands without storing the result. The ZF flag is set if the result of the logical AND is zero, and cleared otherwise.

The following table describes the supported source and destination operands:

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| memory | reg |
| imm | reg |

DO NOT support LOCK

This instruction is only available in compatibility mode. It is used to test a word-sized value in a register against another word-sized value.

The operands MUST be 16-bit. Using this instruction with 32-bit or 64-bit registers will result in an invalid operation or unintended behavior depending on the operand size override prefix. Ensure that the destination register is specifically r16 to avoid size mismatches.
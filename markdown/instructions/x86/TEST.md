> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# TEST

Performs a bitwise logical AND of the source and destination operands. The result is not stored; only the EFLAGS register is updated based on the result.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| mem | reg |
| imm | reg |

DO NOT support LOCK

The instruction is available in both 64-bit mode and compatibility mode. The operand size is determined by the current mode and prefix; in 64-bit mode, the default size is qword unless an operand-size override prefix is used.

The ZF (Zero Flag) is set if the result of the logical AND is zero, and cleared otherwise. The CF (Carry Flag) and OF (Overflow Flag) are cleared. SF (Sign Flag) and PF (Parity Flag) are updated based on the most significant bit and parity of the least significant byte of the result, respectively. Because the destination operand is not modified, TEST is used to check for specific bits without altering the register value.
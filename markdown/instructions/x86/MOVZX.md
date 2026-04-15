> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# MOVZX

MOVZX extends a u8, u16, or u32 source operand to a u32 or u64 destination operand by filling the high-order bits with zeros.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| mN | reg |
| imm | reg |
| #I | mN |

DO NOT support LOCK

MOVZX SHALL NOT be used with a memory destination. In x86-64, the destination MUST be a register. The instruction is available in both 64-bit mode and compatibility mode.

To avoid unintended behavior, ensure the source operand size is strictly smaller than the destination operand size; otherwise, the instruction is invalid. Since MOVZX performs zero-extension, it SHALL only be used for unsigned values. For signed values, MOVSX MUST be used to preserve the sign bit.
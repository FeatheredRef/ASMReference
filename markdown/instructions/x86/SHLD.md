> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# SHLD

SHLD shifts a double precision value formed by two operands to the left. The bits shifted out of the destination operand are shifted into the source operand. The number of bits to shift is specified by a third operand.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| reg | mN |
| #I | imm |

DO NOT support LOCK

SHLD is available in 64-bit mode, 32-bit mode, and 16-bit mode. In 64-bit mode, the instruction supports 8-bit, 16-bit, 32-bit, and 64-bit operand sizes.

If the shift count is specified as an immediate and is greater than or equal to the operand size, the instruction SHALL behave according to the operand size; specifically, for 32-bit operands, the shift count is masked to 5 bits (modulo 32). If the shift count is provided in a register, it is similarly masked. Failure to account for this masking MAY result in unexpected shift amounts when the register value exceeds the operand bit-width.
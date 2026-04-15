> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# SHL

Shifts the destination operand to the left by the number of bits specified by the source operand. Vacant bits are filled with zeros, and the last bit shifted out of the MSB is moved into the CF flag.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| imm | rN |
| reg | rN |
| reg | mN |

DO NOT support LOCK

The instruction is available in 64-bit mode, compatibility mode, and 32-bit mode. When the destination is a 64-bit register, the shift count is masked to 6 bits (0-63) if the source is a register.

If the shift count is equal to or greater than the operand size, the result is defined as zero and CF is undefined. Users SHOULD ensure the shift count is within the range of the operand size to maintain predictable CF behavior. When using an immediate value for the shift count, it MUST be a constant.
> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# MOVSXD

Sign-extends a signed value from a source operand to a destination operand of a larger size.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| mN | reg |
| reg | mN |
| mN | mN |
| imm | #I |

DO NOT support LOCK

This instruction is available in 64-bit mode and compatibility mode. In 64-bit mode, the source operand MUST be a signed value of size 8, 16, or 32 bits, and the destination MUST be a 64-bit register or memory location.

When using `MOVSXD` with a 32-bit source, the instruction sign-extends the `i32` value to an `i64`. Users MUST ensure the source operand is treated as a signed integer; otherwise, the resulting 64-bit value will be incorrect due to the propagation of the sign bit.
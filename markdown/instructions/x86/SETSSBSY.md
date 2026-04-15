> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# SETSSBSY

Sets the destination byte to 1 if the scalar single-precision floating-point value in the source operand is a signaling NaN, and 0 otherwise.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| f32 | reg |
| f32 | m1 |
| imm | #I |
| reg | #I |
| mN | #I |

DO NOT support LOCK

This instruction is available only in 64-bit mode and 32-bit mode; it is not supported in compatibility mode.

The destination operand MUST be a byte-sized register or memory location. Writing to a register larger than 8 bits will result in undefined behavior for the upper bits if not cleared manually. This instruction does not modify the floating-point status register or trigger floating-point exceptions.
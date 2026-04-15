> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# FMUL

Multiplies the destination operand by the source operand. The result is stored in the destination operand.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| memory | reg |
| reg | memory |

DO NOT support LOCK

The instruction is available in 64-bit mode and compatibility mode. It operates on the x87 floating-point stack. If the destination is not specified (implicit), the instruction uses the top of the stack (ST(0)).

When using `FMUL` with a memory operand, the operand MUST be a valid floating-point format (f32 or f64). Using an incorrect data size for the memory operand WILL result in a general protection exception (#GP).

Precision and rounding control are governed by the x87 control word. Results that cannot be represented exactly in the destination format WILL trigger #P. Results that exceed the representable range of the floating-point format WILL trigger #O, and results that are too small WILL trigger #U. Denormalized operands WILL trigger #D.
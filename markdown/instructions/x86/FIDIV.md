> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# FIDIV

Divides an f32 or f64 value by another f32 or f64 value. The destination operand is divided by the source operand, and the quotient is stored in the destination operand.

The following table covers the supported source and destinations.

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| memory | reg |

DO NOT support LOCK

This instruction is used in x86-64 ISA in compatibility mode. In 64-bit mode, the FPU instructions are available, but the architecture prefers the use of SSE/AVX instructions for floating-point operations.

The instruction uses the x87 FPU stack. The destination is implicitly the register `ST(0)`. If a memory operand is used as the source, it is pushed onto the FPU stack before the division occurs.

Users SHALL ensure that the divisor is not zero to avoid a #Z exception. If the result cannot be represented in the destination precision, it MAY trigger #O or #U. Precision errors will result in #P. If any operand is a denormalized value, #D MAY be triggered depending on the FPU control word settings.
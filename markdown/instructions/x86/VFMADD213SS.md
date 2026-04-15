> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VFMADD213SS

Computes the product of two scalar single-precision floating-point values and adds the result to a third scalar single-precision floating-point value. The operation is performed as $f32 = (f32 \times f32) + f32$ with a single rounding step at the end.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| xmm reg, xmm reg, xmm reg | xmm reg |
| xmm reg, xmm reg, m4 | xmm reg |
| xmm reg, m4, xmm reg | xmm reg |
| m4, xmm reg, xmm reg | xmm reg |

DO NOT support LOCK

This instruction SHALL be used in 64-bit mode or 32-bit mode. It is NOT available in compatibility mode if the processor does not support the FMA3 extension. It requires the VEX prefix.

The destination register SHALL NOT be the same as the first source operand (the addend) to avoid destructive updates, although VFMADD213SS is specifically designed as a non-destructive form where the destination can be any of the three operands. Ensure the MXCSR register is configured correctly for rounding mode and exception masking, as this instruction triggers #P, #O, #U, #D, or #I based on the result and inputs.
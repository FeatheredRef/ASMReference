> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# FXAM

Compares two floating-point values (ST(0) and a source operand) and sets the floating-point status word flags based on the result of the comparison.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| f32/f64 | ST(0) |
| m32/m64 | ST(0) |
| ST(i) | ST(0) |

DO NOT support LOCK

This instruction is available in 64-bit mode, compatibility mode, and 32-bit mode. It operates exclusively on the x87 Floating-Point Unit (FPU) register stack.

The source operand must be a valid floating-point number. If the source is a memory operand, the size (m32 or m64) must match the precision of the comparison.

Using FXAM with an invalid operand or a signaling NaN SHALL trigger #I. If the comparison involves denormalized operands, #D MAY be signaled depending on the control word settings. Precision loss during the internal comparison process SHALL trigger #P.
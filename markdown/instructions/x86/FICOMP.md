> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# FICOMP

Compares the f80-bit floating-point value in the ST(0) register with a floating-point value from a source operand. The result of the comparison is stored in the FPU status word.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg | ST(0) |
| mN | ST(0) |

DO NOT support LOCK

This instruction is available in 64-bit mode and compatibility mode. It operates exclusively on the x87 FPU register stack.

The instruction MUST be used with the appropriate precision for the source operand (e.g., `FICOMP` for f80, `FICOMPS` for f64, `FICOMPL` for f32). Using an incorrect suffix for the memory operand size may result in incorrect value interpretation.

If either operand is a Signaling NaN, the instruction SHALL trigger #I. If both operands are Quiet NaNs, #I is triggered. If one operand is a Quiet NaN and the other is a valid number, the comparison result is undefined and #I may be triggered.

The following exceptions may occur:
- #I: Invalid operation.
- #D: Denormalized operand.
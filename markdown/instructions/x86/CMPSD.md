> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# CMPSD

Compares two scalar-single-precision floating-point values by subtracting the source operand from the destination operand. The result is not stored, but the floating-point status flags are updated based on the comparison.

The following table covers what the source and destination can be:

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |

DO NOT support LOCK

This instruction is only available in 64-bit mode or compatibility mode. It requires the SSE extension to be supported by the processor.

The instruction does not modify the values in the xmm registers; it only affects the EFLAGS register (specifically the ZF, PF, and CF flags) based on the comparison of the two f32 values. To avoid incorrect branching, the user MUST use the appropriate conditional jump instruction (e.g., JB, JAE, BEQ) following the comparison. If the operands are NaN, the CF flag is cleared and the ZF flag is set.
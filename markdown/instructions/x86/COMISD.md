> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# COMISD

Compares two scalar-precision floating-point values. The instruction compares the value in the source operand with the value in the destination operand and sets the EFLAGS register (CF, ZF, PF) based on the result of the comparison.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |
| m8 | xmm |

DO NOT support LOCK

This instruction is available in 64-bit mode and compatibility mode. It requires the SSE2 instruction set to be supported by the processor.

If either operand is a Signaling NaN, a floating-point exception is raised. If one operand is a Quiet NaN and the other is not, or if both are Quiet NaNs, the ZF, PF, and CF flags are all set to 1 to indicate an unordered comparison. The user MUST check the PF flag to distinguish between an unordered result (NaN) and a "equal" result.
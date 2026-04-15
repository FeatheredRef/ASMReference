> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# UCOMISS

Compares a scalar-single precision floating-point value with another scalar-single precision floating-point value and sets the EFLAGS register based on the result.

The following table specifies the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| xmm reg | xmm reg |
| m4 | xmm reg |

DO NOT support LOCK

The instruction operates on the lowest 32 bits of the xmm registers. It is supported in both 64-bit mode and compatibility mode.

The instruction does not trigger floating-point exceptions; however, it sets the CF (Carry Flag) if any of the operands are Signaling NaNs or if one operand is a Quiet NaN and the other is not. If both operands are Quiet NaNs, the CF is cleared. Users MUST check the CF status before relying on the ZF (Zero Flag) or PF (Parity Flag) to avoid incorrect logic when dealing with NaN values.
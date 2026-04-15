> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VFNMADD231SS

Computes the fused multiply-add operation $dest = -(a \times b) + c$ for scalar single-precision floating-point values. It multiplies the first two operands, negates the result, and adds it to the third operand.

The following table describes the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| xmm reg, xmm reg, xmm reg | xmm reg |
| xmm reg, m32, xmm reg | xmm reg |
| xmm reg, xmm reg, m32 | xmm reg |

DO NOT support LOCK

This instruction is only available in 64-bit mode or compatibility mode. It requires the FMA3 feature set to be supported by the processor and enabled in the current execution environment.

The result is rounded once at the end of the operation according to the rounding control in the MXCSR register; this prevents intermediate precision loss. If the precision is insufficient, it MAY trigger #P. If the result exceeds the maximum representable value for f32, it SHALL trigger #O. Denormalized inputs MAY trigger #D depending on the MXCSR.FND/MXCSR.ZD settings.
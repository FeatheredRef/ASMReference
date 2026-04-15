> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VFMADD132SH

Computes the fused multiply-add of two scalar single-precision floating-point values and adds the result to a third scalar single-precision floating-point value. The operation is performed as $a = (b \times c) + a$, where the operands are identified by the indices 1, 3, and 2 respectively. The result is rounded once at the end of the entire operation.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| reg, m32 | reg |

DO NOT support LOCK

This instruction is available only in 64-bit mode or compatibility mode. It requires the FMA3 feature set to be supported by the processor and enabled in the operating environment.

The instruction operates on the lowest element of the specified registers. If the instruction is used with XMM registers, the remaining upper bits of the register are undefined. The operation follows IEEE 754 floating-point standards.

Failure to handle floating-point exceptions may lead to unexpected behavior. The instruction may trigger the following:
- #P: If the result is inexact.
- #O: If the result exceeds the maximum representable value.
- #U: If the result is smaller than the minimum representable value.
- #D: If an operand is a denormal.
- #Z: If an operation results in a division by zero (not applicable to multiply-add).
- #I: If an operand is a signaling NaN.
> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VFNMADD132SS

Computes the fused multiply-subtract product for scalar single-precision floating-point values. It calculates $dest = (a \times b) - c$, where the operands are provided in the order defined by the 132 sequence (the 1st and 3rd operands are multiplied, and the 2nd operand is subtracted from the result).

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| reg, reg, reg | reg |
| reg, reg, m32 | reg |
| reg, m32, reg | reg |
| m32, reg, reg | reg |

DO NOT support LOCK

This instruction REQUIRES the AVX support bit to be enabled in the CPU. It is ONLY available in 64-bit mode or compatibility mode.

The instruction uses the destination register as one of the source operands (the subtrahend), meaning the original value of the destination register SHALL be overwritten. To avoid data loss, the destination register MUST be pre-loaded with the value intended for subtraction or backed up.

Precision and rounding are governed by the MXCSR register. The operation MAY trigger the following exceptions: #P, #O, #U, #D, and #Z.
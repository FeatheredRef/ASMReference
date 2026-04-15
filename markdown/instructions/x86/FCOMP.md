> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# FCOMP

Compares the value in the ST(0) register with a floating-point value from the specified source. The result of the comparison is stored in the FPU status word.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg | ST(0) |
| m64 | ST(0) |
| imm | #I |

DO NOT support LOCK

This instruction is available in 64-bit mode and compatibility mode. It operates on the x87 FPU register stack; therefore, it does not utilize the XMM/YMM/ZMM registers.

The instruction does not pop the value from the ST(0) register. To remove the value after comparison, FCOMPP SHALL be used instead. The comparison is subject to the current rounding control in the FPU control word. If the operands are not of the same precision, the smaller precision operand is converted to the higher precision before comparison.

The following exceptions may be triggered:
- #I: Occurs if any operand is a Signaling NaN.
- #D: Occurs if any operand is a denormal.
- #Z: Occurs if the result is zero.
- #O: Occurs if the result cannot be represented in the destination format.
> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# FUCOM

Compares the ST(0) floating-point stack element with a floating-point value. The instruction sets the CPU flags (ZF, PF, CF) based on the result of the comparison to indicate whether the values are equal, unordered, or if one is greater than the other.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg | ST(0) |
| m80 | ST(0) |

DO NOT support LOCK

This instruction is available in 64-bit mode; however, it operates on the x87 FPU register stack.

The result of the comparison depends on the precision of the operand. If the operand is not an extended-precision value, it MUST be converted to extended precision before the comparison occurs. If the comparison results in an undefined value (e.g., comparing a NaN), the FPU status word is updated, and the result is considered unordered.

To avoid incorrect branching, the programmer MUST be aware that `FUCOM` sets the ZF, PF, and CF flags. A result of "unordered" (NaN) is indicated by ZF=1, PF=1, and CF=1. Failure to check for the unordered state before using conditional jumps (like `JE` or `JB`) may lead to incorrect program logic.
> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# FTST

Compares two ST(0) and another floating-point value to set the CPU flags (ZF, PF, CF) according to the result of the comparison. This instruction performs the same operation as FCOM, but does not modify the floating-point status word.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg (f80) | ST(0) |
| m80 | ST(0) |
| ST(i) | ST(0) |

DO NOT support LOCK

The instruction is supported in 64-bit mode and compatibility mode. It is restricted to operations on the x87 FPU stack.

The instruction SHALL be used when the program requires the result of a floating-point comparison to be reflected in the EFLAGS register without altering the FPU status word. Failure to account for the fact that this instruction only compares ST(0) with the source may lead to incorrect logical branching if the stack pointer is not correctly managed.

If any operand is a Signaling NaN, it SHALL trigger #I. If both operands are Quiet NaNs, the ZF, PF, and CF flags SHALL be cleared.
> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# FSUBP

Subtracts the floating-point value in the ST(0) register from the floating-point value in the specified operand, stores the result in the ST(0) register, and then pops the ST(0) register.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| ST(0), reg | ST(0) |
| ST(0), mN | ST(0) |

DO NOT support LOCK

The instruction operates on the x87 floating-point stack. It is available in both 32-bit mode and 64-bit mode (compatibility mode). The operation is subject to the floating-point control word settings regarding rounding and precision.

The following exceptions may occur during execution:
- #I: Invalid operation if either operand is a NaN or if the operation involves an infinity minus an infinity.
- #Z: Divide-by-zero is not applicable to this instruction.
- #D: Denormalized operand if an operand is denormal.
- #O: Numeric overflow if the result exceeds the maximum representable value.
- #U: Numeric underflow if the result is smaller than the minimum representable value.
- #P: Inexact result if the result cannot be represented exactly.

To avoid stack overflow or underflow, the programmer SHALL ensure that the FPU stack pointer (TOP) is in a valid state before execution. Since this instruction pops the stack, the stack depth is decreased by one. Failure to manage the stack pointer may lead to the stack wrapping around.
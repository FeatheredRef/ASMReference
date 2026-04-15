> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# FIADD

Adds the floating-point values of the source and destination operands and stores the result in the destination operand.

The following table covers the supported source and destination operands:

| source | destination(s) |
| :--- | :--- |
| ST(0) | ST(0) |
| mN | ST(0) |

DO NOT support LOCK

This instruction is only available in compatibility mode when executing x87 instructions on x86-64 architectures. It operates specifically on the x87 FPU register stack.

Precision control and rounding mode set in the x87 control word directly affect the result. The operation MAY trigger the following exceptions: #D, #O, #U, #P. If the operation results in a NaN, it SHALL be handled according to the IEEE 754 standard. Since the destination is always the top of the stack (ST(0)), ensure the stack pointer (TOP) is correctly aligned to avoid stack overflow or underflow.
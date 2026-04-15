> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# FSUBRP

Subtracts the floating-point value from the top of the ST(0) register from another floating-point value in the specified register or memory location, then pops the top element from the floating-point register stack.

The following table covers the possible sources and destinations:

| Source | Destination(s) |
| :--- | :--- |
| reg (ST(0)) | reg (ST(i)) |
| reg (ST(0)) | mN |

DO NOT support LOCK

This instruction is available in 64-bit mode and compatibility mode. It specifically operates on the x87 FPU register stack.

The instruction pops the stack after the operation; therefore, the stack pointer (TOP) is decremented by 1. If the stack is empty (TOP=0), the instruction SHALL trigger a stack underflow exception. The result of the subtraction is stored in the destination operand (ST(i) or mN), and the original value of ST(0) is removed.

Precision control and rounding control in the FPU Control Word affect the result. Depending on the operands and result, the following exceptions may be raised: #D, #Z, #O, #U, and #P.
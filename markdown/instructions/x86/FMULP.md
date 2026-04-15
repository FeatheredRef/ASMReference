> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# FMULP

Multiplies the ST(0) floating-point stack element by a floating-point value and pops the operand from the stack.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| mN | reg |
| imm | #I |

DO NOT support LOCK

This instruction is only available in compatibility mode when executing in 64-bit mode. It operates exclusively on the x87 floating-point stack.

The instruction pops the operand from the stack after the operation; therefore, the stack pointer (TOP) is decremented. If the stack is empty, a stack underflow exception occurs. The precision of the operation is determined by the current x87 control word.

The following exceptions may be raised depending on the operands and the control word:
- #I: Invalid operation.
- #Z: Divide-by-zero.
- #D: Denormalized operand.
- #O: Numeric overflow.
- #U: Numeric underflow.
- #P: Inexact result (precision).
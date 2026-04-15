> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# FCOMPP

Compares the ST(0) floating-point stack element with a floating-point value from a source operand. The instruction compares the two values and sets the floating-point condition codes in the FPU status word.

The following table covers the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| reg | ST(0) |
| m64 | ST(0) |
| m32 | ST(0) |

DO NOT support LOCK

This instruction is available in 64-bit mode, but it is primarily used for x87 FPU operations. It operates on the x87 floating-point stack; therefore, the precision of the comparison depends on the current control word setting.

The instruction does not pop the operands from the stack. If the comparison involves a NaN (Not-a-Number), a #P (Inexact result) or an invalid operation exception #I may be signaled depending on the specific NaN type and the FPU control word. Users MUST ensure the FPU stack is not empty to avoid a stack underflow exception.
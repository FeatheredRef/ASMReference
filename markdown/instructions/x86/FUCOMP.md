> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# FUCOMP

Compares two floating-point values (st(0) and a source operand) and sets the Floating-Point Condition Code (FPC) based on the result.

The following table describes the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| reg | st(0) |
| memory | st(0) |

DO NOT support LOCK

This instruction is only available in compatibility mode when executing in a 64-bit environment. It operates on the x87 FPU register stack and does not affect the EFLAGS register.

The result of the comparison is stored in the FPU status word. If the source operand is not a valid floating-point number, the instruction SHALL trigger #I. Precision and underflow exceptions (#P, #U) may be triggered depending on the precision of the operands and the current rounding mode. Note that the source operand is pushed onto the FPU stack if it is a memory or register operand, meaning st(0) becomes the source and the previous st(0) becomes st(1).
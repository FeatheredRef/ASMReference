> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# FCOMIP

Compares the ST(0) floating-point stack element with a floating-point value and updates the floating-point condition codes. Unlike FCOM, this instruction does not modify the CPU flags register (EFLAGS).

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| ST(i) | ST(0) |
| fN (memory) | ST(0) |

DO NOT support LOCK

This instruction is available in both 64-bit mode and compatibility mode. It operates specifically on the x87 Floating-Point Unit (FPU) stack.

The precision of the comparison depends on the current x87 control word. If the precision is set to 64-bit, it performs a double-precision comparison; if set to 80-bit, it performs an extended-precision comparison. Since FCOMIP does not affect the EFLAGS register, developers MUST use `FSTS` or `FCOMIP` in conjunction with `FCOMIP`'s status word (accessed via `FYREGS` or `FSTSW`) to evaluate the result of the comparison. Failure to check the FPU status word will result in an inability to determine the outcome of the operation.
> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# FCOMI

Compares f80 with another x87 floating-point value and sets the EFLAGS CPU flags (CF, ZF, PF) based on the result of the comparison.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| reg | st(0) |
| memory | st(0) |

DO NOT support LOCK

This instruction is only available in 32-bit mode and 64-bit mode. It operates exclusively on the x87 FPU register stack.

The instruction SHALL be used with caution regarding the floating-point control word; it does not affect the x87 status word but directly modifies the EFLAGS register, which is a departure from standard x87 comparison instructions that typically only set the FPU status word. If the comparison results in an undefined value (e.g., involving NaNs), the ZF flag SHALL be set to 1 and the CF flag SHALL be set to 1. Failure to handle NaN cases may lead to incorrect branching logic in the application.
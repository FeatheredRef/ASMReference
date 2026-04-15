> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# FNCLEX

Clears the current floating-point exception flag in the x87 floating-point status word.

The following table covers the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| #I | #I |

DO NOT support LOCK

This instruction is available in 32-bit protected mode, 64-bit mode, and compatibility mode. It does not take any operands and operates exclusively on the x87 FPU status word.

The FNCLEX instruction only clears the exception flag; it does not affect the exception mask bits or the condition code flags. To clear all exception flags and the status word entirely, FINIT SHOULD be used instead. Failure to clear exception flags before subsequent floating-point operations may lead to the unexpected triggering of exception handlers if the corresponding mask bits are unset.
> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# FLDL2T

Calculates $st(1) = st(1) \times \log_2(st(0))$ and pops the value at $st(0)$ from the floating-point register stack.

The following table covers the source and destinations:

| source | destination(s) |
| :--- | :--- |
| reg | #I |
| memory | #I |
| imm | #I |
| f80 | st(1) |

DO NOT support LOCK

This instruction is available in 64-bit mode and compatibility mode. It operates exclusively on the x87 FPU register stack; therefore, it is not applicable to XMM or YMM registers.

The instruction pops $st(0)$, which decrements the FPU TOP pointer. If the FPU stack is empty or contains only one element, a stack underflow exception may occur. Ensure that at least two values are present on the stack before execution to avoid `#I`. Precision and rounding control are governed by the FPU control word, and results may trigger `#P`, `#U`, or `#O` depending on the magnitude of the operands.
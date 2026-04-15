> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VFMADDSUB132PD

Performs a fused multiply-add or multiply-subtract operation on packed double-precision floating-point values. Depending on the immediate operand, it computes either $r64a = (r64a \times r64b) + r64c$ or $r64a = (r64a \times r64b) - r64c$. The "132" notation indicates that the first operand is multiplied by the second, and the result is added to or subtracted from the third.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| imm, reg, reg, reg | reg |
| imm, reg, reg, m128/m256/m512 | reg |
| imm, reg, m128/m256/m512, reg | reg |
| imm, reg, m128/m256/m512, m128/m256/m512 | reg |

DO NOT support LOCK

This instruction SHALL be executed in 64-bit mode or 32-bit mode; it is NOT available in compatibility mode if the processor does not support the corresponding AVX/AVX2/AVX-512 extensions. The instruction requires the YMM or ZMM registers; if used with XMM registers, it operates as a subset of the AVX-512 foundation.

The instruction SHALL trigger floating-point exceptions including #P, #O, #U, #D, and #Z based on the result of the fused operation. Because it is a fused operation, only one rounding step is performed at the end, which prevents intermediate precision loss. Users MUST ensure that the appropriate AVX or AVX-512 CPUID flags are checked before execution to avoid `#UD` (Undefined Instruction) exceptions. When using ZMM registers, the upper bits of the destination register are zeroed if the operand size is smaller than the register size (masking behavior applies in EVEX encoded versions).
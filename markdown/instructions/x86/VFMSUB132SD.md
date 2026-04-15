> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VFMSUB132SD

Performs a fused multiply-subtract operation on scalar double-precision floating-point values. The instruction computes the result of $a \times b - c$ and stores it in the destination, where $a$ is the first source operand, $b$ is the second source operand, and $c$ is the third source operand.

The following table describes the supported source and destination operands:

| source | destination(s) |
| :--- | :--- |
| f64 (reg/mem) | f64 (reg) |
| f64 (reg) | f64 (reg) |
| f64 (reg) | f64 (reg) |

DO NOT support LOCK

This instruction is only available when the processor supports the AVX-512 foundation. It requires the execution environment to be in 64-bit mode or a compatibility mode that supports AVX-512 instructions.

The operation is performed as a single fused step with only one rounding stage at the end, which prevents intermediate precision loss and avoids the #P exception that would occur if a separate multiply and subtract were used. If any operand is a signaling NaN, a #S exception may be raised. The result is subject to standard IEEE 754 rounding modes as defined in the MXCSR register.
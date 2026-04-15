> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VFNMADD132SD

Performs a fused multiply-add operation on scalar double-precision floating-point values. It computes the result as $-(a \times b) + c$, where $a$ is the first operand, $b$ is the second operand, and $c$ is the third operand.

The following table describes the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| f64, f64, f64 | f64 |
| f64, f64, m8 | f64 |
| f64, m8, f64 | f64 |
| m8, f64, f64 | f64 |

DO NOT support LOCK

This instruction is only available in 64-bit mode. It REQUIRES the AVX instruction set support.

The instruction is subject to the floating-point control word and MXCSR register settings. Results are rounded according to the rounding mode specified in the MXCSR register. The operation may trigger floating-point exceptions: #O if the result overflows, #U if the result underflows, and #P if the result is inexact.

To avoid precision loss or unexpected results, ensure that the destination register is not used as a source operand unless the intended behavior is to overwrite the original value. When using memory operands, the memory address MUST be naturally aligned to an 8-byte boundary to avoid performance degradation or alignment faults depending on the processor configuration.
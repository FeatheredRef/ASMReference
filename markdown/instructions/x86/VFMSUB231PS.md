> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VFMSUB231PS

Performs a fused multiply-subtract operation on packed single-precision floating-point values. The operation computes the result of $(a \times c) - b$ for each corresponding element of the input vectors, where $a$ is the first source operand, $b$ is the second source operand, and $c$ is the third source operand.

The following table describes the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| reg | reg |
| m32 | reg |

DO NOT support LOCK

This instruction is only available when the processor supports the AVX-512 foundation instructions. It requires the processor to be operating in 64-bit mode or compatibility mode.

The operation is performed as a single floating-point step with one final rounding, which prevents the loss of precision that occurs when performing a multiply followed by a separate subtract. If the destination register is also used as a source, the original values are overwritten. Users SHALL ensure that the appropriate AVX-512 feature flags are checked via CPUID before execution to avoid invalid opcode exceptions.

The result of the operation may trigger floating-point exceptions: #D if a denormalized operand is encountered, #O if the result overflows, #U if the result underflows, and #P if the result is inexact.
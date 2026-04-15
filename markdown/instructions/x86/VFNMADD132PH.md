> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VFNMADD132PH

Computes a fused multiply-subtract operation on half-precision floating-point values. The instruction calculates the result as $(a \times c) - b$ and stores it in the destination.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| reg | reg |

DO NOT support LOCK

This instruction is available only in 64-bit mode or compatibility mode. It requires the AVX-512 FP16 extension to be supported by the processor.

The operation is performed using the precision specified by the half-precision (16-bit) floating-point format. The intermediate product $(a \times c)$ is calculated with infinite precision before the subtraction of $b$, avoiding intermediate rounding errors. This may trigger the following floating-point exceptions: #D, #O, #U, and #P.
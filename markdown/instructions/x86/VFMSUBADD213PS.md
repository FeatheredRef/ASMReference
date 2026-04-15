> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VFMSUBADD213PS

Performs a fused multiply-subtract and add operation on packed single-precision floating-point values. The instruction calculates `(a * b) - c + d` for each corresponding element of the input vectors. Specifically, it computes the product of the first two source operands, subtracts the third source operand from that product, and then adds the fourth source operand, storing the result in the destination.

The following table covers the supported source and destination operands:

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| reg | m32 |
| m32 | reg |
| m32 | m32 |

DO NOT support LOCK

This instruction is available only in 64-bit mode or compatibility mode. It requires the AVX-512 foundation extensions to be enabled.

The operation is performed according to the IEEE 754 standard. The fused multiply-subtract step is performed with infinite precision before the final addition and rounding.

Precision and exception flags are affected as follows:
- #P: Set if the rounded result of the operation differs from the exact result.
- #O: Set if the result exceeds the maximum representable value of f32.
- #U: Set if the result is non-zero but smaller in magnitude than the smallest representable normalized f32.
- #D: Set if any operand is a denormalized value or if the result is denormalized.

Users MUST ensure that the appropriate AVX-512 feature flags are enabled in the CPU via CR4 and that the OS supports the state saving for the ZMM registers to avoid undefined behavior or general protection faults.
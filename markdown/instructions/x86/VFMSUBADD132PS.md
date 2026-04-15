> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VFMSUBADD132PS

Performs a fused multiply-subtract and add operation on packed single-precision floating-point values. It computes the result of $(a \times b) - c + d$ for each corresponding element of the input vectors, where $a$, $b$, $c$, and $d$ are the operands.

The following table specifies the supported sources and destinations.

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| reg | m128 |
| m128 | reg |
| m128 | m128 |

DO NOT support LOCK

This instruction is available only in 64-bit mode or compatibility mode. It requires the AVX-512 foundation and the FMA extensions.

The operation is subject to the floating-point rounding mode specified in the MXCSR register. If any operand is a signaling NaN, a #P exception may be generated. Precise result calculation follows IEEE 754 standards, meaning the multiply-subtract occurs as a single operation with one rounding step before the final addition. Failure to ensure the destination register does not overlap with source registers in non-destructive forms may lead to undefined behavior in certain assembler contexts, although the VEX/EVEX encoding typically handles this via three-operand syntax.
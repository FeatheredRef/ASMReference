> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VFMSUB231SD

Subtracts the product of two double-precision floating-point values from a third double-precision floating-point value. The operation is performed as: $dest = src1 - (src2 \times src3)$.

The following table covers the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| f64, f64, f64 | f64 |
| reg, reg, reg | reg |
| m8, reg, reg | reg |

DO NOT support LOCK

This instruction SHALL only be executed on processors that support the AVX-512 Fused Multiply-Add (FMA) instruction set. It is available in both 64-bit mode and compatibility mode.

To avoid precision loss or unexpected results, the user MUST ensure that the destination register is not used as one of the source operands if the operation is intended to be non-destructive, although this instruction is designed for three-operand destructive or non-destructive forms depending on the encoding. Failure to handle floating-point exceptions may result in #P, #O, #U, #D, or #Z depending on the input values and the MXCSR register settings.
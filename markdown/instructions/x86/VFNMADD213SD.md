> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VFNMADD213SD

Computes the fused multiply-add operation for scalar double-precision floating-point values. It calculates the result using the formula: $dest = -(a \times b) + c$, where $a$, $b$, and $c$ are the operands.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| f64, f64, f64 | f64 |
| reg, reg, reg | reg |
| reg, reg, m8 | reg |
| reg, m8, reg | reg |
| m8, reg, reg | reg |

DO NOT support LOCK

This instruction SHALL only be executed in 64-bit mode or compatibility mode. It requires the AVX extension to be enabled in the processor.

The instruction utilizes the Fused Multiply-Add (FMA) unit; therefore, it performs the multiplication and addition as a single operation with only one rounding step at the end. This prevents intermediate precision loss.

The operation MAY trigger the following exceptions:
- #D: If any operand is a denormalized value.
- #O: If the result exceeds the maximum representable value for f64.
- #U: If the result is too small to be represented as a normalized f64.
- #P: If the rounded result differs from the infinitely precise result.

The destination register is used as one of the source operands (the addend $c$). If the destination register is also used as a multiplier, the value is read before the result is written.
> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VFMSUBADD231PD

Performs a fused multiply-subtract and add operation on double-precision floating-point values. It computes the result of (a * b) - c + d for the first element of the vector and (a * b) + c - d for the second element, where a, b, c, and d are the input operands.

The following table covers the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| f64, f64, f64, f64 | f64 |
| reg, reg, reg, reg | reg |
| reg, reg, reg, m128 | reg |
| reg, reg, m128, reg | reg |
| reg, m128, reg, reg | reg |

DO NOT support LOCK

This instruction is available only when the AVX-512 foundation instructions are supported. It requires the processor to be in 64-bit mode or compatibility mode.

The operation is performed with a single rounding step at the end, which prevents the loss of precision associated with intermediate rounding. Users SHOULD ensure that the processor supports the AVX-512F feature set to avoid `#UD` (Undefined Instruction) exceptions.

The instruction may trigger the following floating-point exceptions:
- `#O`: If the result exceeds the maximum representable value.
- `#U`: If the result is smaller than the minimum representable value.
- `#P`: If the result is not exactly representable.
- `#D`: If a denormalized operand is encountered.
> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VCVTSD2USI

Converts a scalar double-precision floating-point value to an unsigned 64-bit integer. The conversion uses the rounding mode specified in the MXCSR register. If the source value is NaN or exceeds the range of a 64-bit unsigned integer, a #S (Invalid Operation) exception is generated and the destination is set to the indeterminate value.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| f64 | r64 |
| m8 | r64 |

DO NOT support LOCK

This instruction is only available in 64-bit mode. It SHALL NOT be used in compatibility mode.

If the source operand is a signaling NaN, a #S exception is raised. If the resulting unsigned integer is out of range (greater than $2^{64}-1$ or less than 0), a #S exception is raised. Precision (#P) is raised if the result is not exact.
> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VFMADD231PH

Multiplies a packed set of half-precision floating-point values (f16) from the first source operand by a packed set of half-precision floating-point values from the second source operand, adds the result to a packed set of half-precision floating-point values from the third source operand, and stores the result in the destination operand. The operation follows the form: `dest = (src1 * src2) + src3`.

The following table covers the supported source and destination operands.

| Source | Destination(s) |
| :--- | :--- |
| reg, reg, reg | reg |
| reg, reg, m16 | reg |
| reg, m16, reg | reg |
| m16, reg, reg | reg |

DO NOT support LOCK

This instruction SHALL only be executed in 64-bit mode or 32-bit mode. It requires the AVX-512 Fused Multiply-Add (FMA) and AVX-512 FP16 extensions to be enabled in the processor.

To avoid precision loss or unexpected behavior, the user SHALL ensure that the floating-point control word is correctly configured. The instruction produces results based on the current rounding mode. Failure to handle floating-point exceptions may result in `#I`, `#Z`, `#D`, `#O`, `#U`, or `#P` being set in the MXCSR register. Use of this instruction on non-aligned memory operands may result in a general-protection exception if alignment checks are enabled.
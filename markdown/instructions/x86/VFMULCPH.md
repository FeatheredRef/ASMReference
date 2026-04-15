> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VFMULCPH

Multiplies converted half-precision floating-point values. The instruction converts the half-precision floating-point operands to single-precision floating-point format, performs the multiplication, and then converts the result back to half-precision floating-point format.

The following table covers the supported sources and destinations:

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| m16 | reg |

DO NOT support LOCK

The instruction is available only in 64-bit mode. It requires the AVX-512 FP16 extension.

The result is rounded according to the rounding mode specified in the MXCSR register. If the converted result cannot be represented as a half-precision floating-point number, it MAY result in #O or #U. Precision loss during the conversion from single-precision back to half-precision SHALL trigger #P.
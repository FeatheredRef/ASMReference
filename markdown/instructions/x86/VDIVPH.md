> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VDIVPH

Divides packed half-precision floating-point values. The instruction divides the half-precision floating-point values in the source operands and stores the result in the destination.

The following table covers the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| xmm / ymm / zmm | xmm / ymm / zmm |
| m16 / m32 / m64 | xmm / ymm / zmm |

DO NOT support LOCK

This instruction is available only in 64-bit mode or compatibility mode. It requires the AVX-512 FP16 extension.

The operation follows the IEEE 754 half-precision binary floating-point format. If the divisor is zero and the dividend is a finite non-zero number, #Z is signaled. If both operands are zero, #I is signaled. Results that exceed the representable range of the half-precision format trigger #O, while results smaller than the smallest representable normal number may trigger #U or #D depending on the rounding mode. Any loss of precision during the division will signal #P.
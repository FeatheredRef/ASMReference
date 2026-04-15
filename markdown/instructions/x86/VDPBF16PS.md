> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VDPBF16PS

Converts packed bfloat16 floating-point values to packed single-precision floating-point values. The instruction converts each bfloat16 value in the source to a 32-bit single-precision float by appending zeros to the mantissa.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| xmm/ymm/zmm | xmm/ymm/zmm |
| m16/m32/m64 | xmm/ymm/zmm |

DO NOT support LOCK

This instruction is available only in 64-bit mode or compatibility mode. It requires the AVX-512 BF16 extension.

The instruction performs a widening conversion; therefore, the destination register must be large enough to hold the resulting single-precision floats. If the source is a ZMM register, the operation is performed on the lower half of the register to accommodate the increase in bit-width per element. Failure to account for the element size increase may result in accessing unexpected register lanes.
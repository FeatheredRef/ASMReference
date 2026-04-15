> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VCMPSH

Compares the packed single-precision floating-point values in the first source operand with the packed single-precision floating-point values in the second source operand using the specified predicate. The result is stored in the destination operand.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| xmm / ymm / zmm | xmm / ymm / zmm |
| m16 / m32 / m64 | xmm / ymm / zmm |
| imm | xmm / ymm / zmm |

DO NOT support LOCK

This instruction is available in 64-bit mode and compatibility mode. It requires the AVX and AVX2 instruction set extensions.

The destination register is overwritten by the comparison mask; failure to use a separate destination register for the mask will result in the loss of the original input data. When using the immediate operand for the predicate, the specific comparison type (e.g., Equal, Not Equal, Greater Than) is determined by the bits provided in the immediate. When using a memory operand, the memory alignment MUST follow the requirements for the vector size (xmm, ymm, or zmm) to avoid performance penalties or faults unless unaligned move support is enabled.

The instruction MAY trigger the following floating-point exceptions based on the comparison predicate and the values of the operands: #D, #O, #U, #P.
> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VPSHLD

Shifts the elements of a packed YMM or ZMM register to the left by a count specified by another register, filling the vacated bits with zeros.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| #I | mN |

DO NOT support LOCK

This instruction SHALL be used only in 64-bit mode or compatibility mode. It requires AVX-512 support (specifically AVX-512F) to be enabled in the processor.

The shift count is taken from the low-order bits of the source register; for a 32-bit element, only the bits 0-31 are used. If the shift count is greater than or equal to the element size, the result SHALL be zero. Using a memory operand for the destination is NOT supported.
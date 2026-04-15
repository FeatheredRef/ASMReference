> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# TDPBUUD

This instruction converts a packed double-precision floating-point value to an unsigned 64-bit integer value with rounding toward zero (truncation).

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| m8 | #I |
| m16 | #I |
| m32 | #I |
| m64 | #I |
| m128 | #I |
| reg | reg |
| imm | #I |

DO NOT support LOCK

This instruction SHALL only be available in 64-bit mode or compatibility mode. It REQUIRES support for the AVX-512 extensions.

If the source value is NaN, the destination register SHALL be set to an indefinite value. If the source value is too large to be represented as a u64, the result SHALL be the maximum representable value for a u64.
> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VGETMANTSD

Extracts the mantissa from the lowest scalar double-precision floating-point value of a ZMM register and stores it as a 64-bit integer in a general-purpose register.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| zmm | r64 |

DO NOT support LOCK

This instruction is only available in 64-bit mode. It requires AVX-512 support.

The destination register `r64` is overwritten with the 52-bit mantissa of the scalar double-precision value; the upper 12 bits of the `r64` register are zeroed. This operation does not trigger floating-point exceptions (#Z, #D, #O, #U, #P). Failure to ensure the processor is in 64-bit mode SHALL result in an invalid opcode exception.
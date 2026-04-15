> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VPRORVD

Rotates the unsigned 32-bit integers in a destination vector to the right by the number of positions specified by an immediate value or a source vector. The rotation is performed on each 32-bit element independently.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| imm | reg |

DO NOT support LOCK

This instruction is available only in 64-bit mode or 32-bit mode when AVX-512 is supported. It requires the AVX-512F instruction set. It does not operate in compatibility mode if the processor does not support the corresponding AVX-512 extensions.

The rotation count is masked to 31 (u31) if a register is used as the rotation amount; values exceeding 31 will be truncated. Using an immediate value allows for a constant rotation across all elements. Failure to account for the 31-bit mask when using a register may result in an unintended rotation amount.
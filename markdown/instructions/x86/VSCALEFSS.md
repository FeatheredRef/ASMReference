> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VSCALEFSS

Scales a scalar single-precision floating-point value by a specified scalar multiplier and stores the result in a destination.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| f32 | f32 |
| f32 | f32 |
| imm | f32 |

DO NOT support LOCK

This instruction is only available when the processor supports the AVX-512 foundation. It requires the processor to be operating in 64-bit mode or compatibility mode.

The scaling factor is provided as an immediate value that represents a power of 2. The operation performs a multiplication of the source value by $2^{imm}$, where the immediate is treated as a signed integer. If the result cannot be represented as a single-precision floating-point number, the processor SHALL signal #O or #U depending on the magnitude of the result. Precision exceptions (#P) SHALL be signaled if the result is rounded.
> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VREDUCESS

Reduces the precision of a floating-point value from a double-precision format to a single-precision format.

The following table covers the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| f64 | f32 |

DO NOT support LOCK

This instruction is only available when the processor is operating in a mode that supports AVX-512 and the specific feature flag for the corresponding subset is enabled.

The instruction performs rounding according to the rounding control word in the MXCSR register. If the source value is too large to be represented as a single-precision float, the operation SHALL signal #O and the result SHALL be the signed infinity of the corresponding sign. If the result is too small to be represented as a normalized single-precision float, the operation SHALL signal #U. Any result that cannot be represented exactly in the destination format SHALL signal #P.
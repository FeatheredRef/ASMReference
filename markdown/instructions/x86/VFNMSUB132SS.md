> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VFNMSUB132SS

Subtracts the second operand from the first operand, negates the result, and stores the result in the destination. This instruction operates on the lowest 32 bits of the scalar floating-point operands.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| f32 | f32 |
| f32 | f32 |
| f32 | f32 |

DO NOT support LOCK

This instruction requires the AVX instruction set to be supported by the processor. It is available in 64-bit mode and compatibility mode.

The destination register is overwritten by the result; therefore, it cannot be used as a read-only source if the original value must be preserved. When using the three-operand form, the destination is independent of the sources.

The operation is subject to x87-style floating-point exception flags. Specifically, it may trigger #P if the result is inexact, or #O if the result exceeds the maximum representable value for a 32-bit float. Denormal operands may trigger #D depending on the MXCSR register settings (Flush-to-Zero or Denormals-Are-Zero).
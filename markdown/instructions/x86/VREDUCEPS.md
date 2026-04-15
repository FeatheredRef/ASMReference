> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VREDUCEPS

Reduces a set of four single-precision floating-point values to a single value by performing a pairwise addition. The instruction takes a source operand containing four f32 values and produces a single f32 result.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| xmm reg | xmm reg |

DO NOT support LOCK

This instruction is available only in 64-bit mode and compatibility mode. It requires the AVX extension to be supported by the processor.

To avoid undefined behavior or precision loss, ensure that the destination register does not overlap with the source register if the specific implementation requires non-destructive operands. The operation is subject to floating-point exception flags; specifically, it MAY trigger #O, #U, or #P depending on the input values and the current MXCSR rounding mode.
> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VINSERTF64x2

Inserts two packed single-precision floating-point values from the source into a destination vector of packed single-precision floating-point values. The insertion occurs at the index specified by the immediate operand.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm-reg | xmm-reg |
| m64 | xmm-reg |

DO NOT support LOCK

This instruction is only available when the processor is operating in 64-bit mode or compatibility mode. It requires the AVX instruction set to be enabled.

The immediate operand MUST specify a valid index; if the index is out of bounds for the destination vector size, the behavior is undefined or may trigger an exception depending on the specific processor implementation. This instruction does not trigger any floating-point exceptions (#I, #Z, #D, #O, #U, #P) as it performs a data movement operation rather than an arithmetic calculation.
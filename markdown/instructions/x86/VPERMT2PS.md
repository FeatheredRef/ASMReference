> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VPERMT2PS

This instruction permutes two packed single-precision floating-point vectors. It selects four single-precision floating-point values from the source registers based on an immediate byte and stores the result in the destination register.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm reg, xmm reg, imm | xmm reg |

DO NOT support LOCK

This instruction is only available in 64-bit mode or compatibility mode. It requires the AVXK (AVX) instruction set extension.

The immediate value consists of four 2-bit indices. Each index selects a `f32` element from either the first or second source register. An index of 0-3 selects from the first source, and 4-7 selects from the second source. If the immediate value specifies an index that exceeds the available elements in the combined source registers, the behavior is undefined. Failure to ensure the immediate is within the valid range for the provided `xmm` registers may lead to unexpected results.
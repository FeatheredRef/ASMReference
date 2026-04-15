> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VPERMI2D

This instruction selects four double words from two source operands based on an immediate value and stores the result in the destination register. The immediate specifies which double word from the first source (zmm1) or second source (zmm2) is placed into each of the four double word slots of the destination.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| mem | reg |

DO NOT support LOCK

This instruction SHALL only be executed in 64-bit mode. It REQUIRES the AVX-512 foundation (AVX-512F) support. The instruction does not support memory-to-memory operations.

The destination register MUST NOT be used as the second source operand if the first source is a memory operand to avoid potential write-after-read hazards depending on the microarchitecture, although the ISA allows it. Ensure that the immediate value is within the valid range for a 4-slot selection; values outside the specified encoding for VPERMI2D are invalid.
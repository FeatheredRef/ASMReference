> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VFPCLASSSH

Classifies the upper double-precision floating-point value of a 128-bit YMM register and stores the resulting classification in a general-purpose register.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg | reg |

DO NOT support LOCK

This instruction is only available when the processor is operating in 64-bit mode or compatibility mode. It requires the processor to support the AVX instruction set.

The destination register must be a 32-bit general-purpose register (r32). The result is an u32 value representing the classification of the floating-point operand according to the IEEE 754 standard. Ensure that the YMM register is properly initialized; otherwise, the classification will reflect the existing data in the upper 64 bits of the register.
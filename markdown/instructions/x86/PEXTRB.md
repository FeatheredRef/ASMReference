> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PEXTRB

Extracts a byte from a source operand at a specified index and stores it into the destination operand.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| m8, r8 | r8 |
| m16, r16 | r8 |
| m32, r32 | r8 |
| m64, r64 | r8 |

DO NOT support LOCK

This instruction is available only in 64-bit mode. It requires the BMI1 instruction set extension to be supported by the processor.

The index of the byte to be extracted is specified by an immediate value. This immediate MUST be an unsigned integer. If the index is out of range for the size of the source operand, the behavior is undefined. The destination register is zero-extended to the size of the register (r8).
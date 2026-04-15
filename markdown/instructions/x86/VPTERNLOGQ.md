> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VPTERNLOGQ

Computes a bitwise logical operation on three 64-bit quad-word operands according to a specified immediate value. The operation is performed for each element in the zmm registers, where the immediate serves as a selection table for the truth table of the desired logical function.

The following table covers the supported sources and destinations:

| source | destination(s) |
| :--- | :--- |
| imm | #I |
| reg | reg |
| m8 | reg |

DO NOT support LOCK

This instruction is only available in 64-bit mode. It requires the AVX-512 foundation extensions to be enabled.

The immediate operand MUST be a u8 value; bits 0-7 define the logical operation. If the instruction is used with masked registers, elements that are not selected by the opmask are either zeroed or left unchanged based on the masking policy (z or m). Failure to properly initialize the mask register may result in undefined behavior for the inactive elements of the destination register.
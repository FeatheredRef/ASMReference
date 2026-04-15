> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VPEXPANDQ

Expands a packed sequence of 64-bit integers from a source vector to a destination vector by inserting zeros between the original elements based on a specified expansion factor.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| zmmN | zmmN |
| m64 | zmmN |

DO NOT support LOCK

This instruction SHALL be executed only in 64-bit mode. It requires AVX-512 support and specifically the AVX-512DQ extension.

The destination register SHALL NOT be the same as the source memory operand. If the destination register is used as a source, the operation is performed in-place. The expansion factor is encoded in the immediate operand; if the immediate is invalid or unsupported for the specific vector length, the instruction SHALL trigger an invalid opcode exception. Alignment of memory operands SHALL follow the standard AVX-512 alignment rules to avoid performance penalties or faults.
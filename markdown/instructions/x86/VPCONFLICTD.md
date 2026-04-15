> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VPCONFLICTD

Detects conflicts between 32-bit signed integer elements within a vector. It compares each element in the source vector with all previous elements in the same vector and sets a bit in the destination register indicating if a conflict (equality) was found.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| m32 | reg |

DO NOT support LOCK

This instruction is only available in 64-bit mode. It requires the AVX-512 Conflict Detection Foundation (AVX-512CD) extension to be supported by the processor.

The destination register is overwritten. If the instruction is used with masking, the masking behavior is applied to the destination; elements not updated by the mask retain their previous values. Note that the conflict detection logic is based on the values present in the source operand regardless of the mask, but the mask controls which results are written to the destination.
> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VSHUFI64x2

This instruction shuffles two 64-bit elements from a source vector to a destination vector based on an immediate byte. The immediate value specifies which 64-bit element from the first or second source operand is selected for each 64-bit lane of the destination.

The following table describes the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| xmm reg | xmm reg |
| xmm mem | xmm reg |

DO NOT support LOCK

This instruction is only available in 64-bit mode or compatibility mode. It is NOT supported in 32-bit mode.

The immediate operand must be a valid shuffle control byte. If the immediate value specifies an index that exceeds the available elements in the source register (beyond the 64-bit granularity), the behavior is defined by the specific shuffle pattern mapping in the Intel SDM. Using an unsupported immediate value may result in undefined behavior or specific masking depending on the processor implementation.
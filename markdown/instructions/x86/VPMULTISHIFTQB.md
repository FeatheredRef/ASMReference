> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VPMULTISHIFTQB

Multiplies unsigned 8-bit integers from a source operand by a set of constants defined in an immediate, then shifts the results by a set of constants defined in the same immediate, and stores the result in the destination operand.

The following table describes the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| m128 | xmm |
| xmm | xmm |

DO NOT support LOCK

This instruction is available only in 64-bit mode. It requires the AVX-512 VBMI (Vector Byte Manipulation Instructions) subset.

The immediate operand defines the shift and multiply parameters for each byte lane. Users SHALL ensure that the processor supports the VBMI feature flag to avoid an invalid opcode exception. Failure to align memory operands to 64-byte boundaries may result in performance degradation, although it is not architecturally required for correctness.
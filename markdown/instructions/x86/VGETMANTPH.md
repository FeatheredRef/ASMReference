> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VGETMANTPH

Extracts the mantissa (fraction) from a packed set of half-precision floating-point values.

The following table specifies the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| xmm/ymm/zmm register | xmm/ymm/zmm register |

DO NOT support LOCK

This instruction is only available if the processor supports the AVX-512 FP16 extension. It operates on packed f16 values; using this instruction on hardware without FP16 support SHALL result in an invalid opcode exception.

To avoid incorrect results, the destination register MUST be of the same vector length as the source register to prevent data corruption or unexpected behavior across the vector lanes. The extracted mantissa is typically zero-extended to the destination register's element size.
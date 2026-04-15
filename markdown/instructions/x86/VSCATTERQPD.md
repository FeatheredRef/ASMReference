> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VSCATTERQPD

Stores 64-bit floating-point values from a zmm register into non-contiguous memory locations specified by indices in a zmm register.

The following table specifies the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| reg | m64 |

DO NOT support LOCK

This instruction is only available when AVX-512 is supported. It requires a 64-bit operating environment (Long Mode).

Masking is MANDATORY for this instruction. Only elements corresponding to set bits in the opmask register are stored; elements corresponding to clear bits are ignored.

Memory faults are suppressed for elements where the corresponding mask bit is 0. This allows the instruction to be used with indices that may point to invalid memory addresses, provided the mask prevents the access.

The behavior of the instruction is undefined if the same index is specified multiple times within a single instruction. To avoid data races or non-deterministic results, the programmer MUST ensure that indices are unique.
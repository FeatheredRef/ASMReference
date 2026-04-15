> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VCVTW2PH

Converts a signed word integer value to a half-precision floating-point value.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| m2 | reg |

DO NOT support LOCK

This instruction is available only in 64-bit mode. It requires the AVX-512 FP16 instruction set extension.

The operation is subject to the following details:
- The conversion follows the IEEE 754 standard for rounding.
- If the source value is too large to be represented as a half-precision float, the result is set to $\pm\infty$ and #O is signaled.
- Precision loss during conversion will signal #P.